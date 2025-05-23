/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use dpi::PhysicalSize;
use raw_window_handle::{DisplayHandle, RawDisplayHandle, RawWindowHandle, WindowHandle};
pub use servo::webrender_api::units::DeviceIntRect;
use servo::{self, EventLoopWaker, ServoBuilder, resources};
pub use servo::{InputMethodType, MediaSessionPlaybackState, WindowRenderingContext};

use crate::egl::android::resources::ResourceReaderInstance;
#[cfg(feature = "webxr")]
use crate::egl::app_state::XrDiscoveryWebXrRegistry;
use crate::egl::app_state::{Coordinates, RunningAppState, ServoWindowCallbacks};
use crate::egl::host_trait::HostTrait;
use crate::prefs::{ArgumentParsingResult, parse_command_line_arguments};

thread_local! {
    pub static APP: RefCell<Option<Rc<RunningAppState>>> = const { RefCell::new(None) };
}

pub struct InitOptions {
    pub args: Vec<String>,
    pub url: Option<String>,
    pub coordinates: Coordinates,
    pub density: f32,
    #[cfg(feature = "webxr")]
    pub xr_discovery: Option<servo::webxr::Discovery>,
    pub window_handle: RawWindowHandle,
    pub display_handle: RawDisplayHandle,
}

/// Initialize Servo. At that point, we need a valid GL context.
/// In the future, this will be done in multiple steps.
pub fn init(
    mut init_opts: InitOptions,
    waker: Box<dyn EventLoopWaker>,
    callbacks: Box<dyn HostTrait>,
) -> Result<(), &'static str> {
    crate::init_crypto();
    resources::set(Box::new(ResourceReaderInstance::new()));

    // `parse_command_line_arguments` expects the first argument to be the binary name.
    let mut args = mem::take(&mut init_opts.args);
    args.insert(0, "servo".to_string());

    let (opts, preferences, servoshell_preferences) = match parse_command_line_arguments(args) {
        ArgumentParsingResult::ContentProcess(..) => {
            unreachable!("Android does not have support for multiprocess yet.")
        },
        ArgumentParsingResult::ChromeProcess(opts, preferences, servoshell_preferences) => {
            (opts, preferences, servoshell_preferences)
        },
    };

    crate::init_tracing(servoshell_preferences.tracing_filter.as_deref());

    let (display_handle, window_handle) = unsafe {
        (
            DisplayHandle::borrow_raw(init_opts.display_handle),
            WindowHandle::borrow_raw(init_opts.window_handle),
        )
    };

    let size = init_opts.coordinates.viewport.size;
    let rendering_context = Rc::new(
        WindowRenderingContext::new(
            display_handle,
            window_handle,
            PhysicalSize::new(size.width as u32, size.height as u32),
        )
        .expect("Could not create RenderingContext"),
    );

    let window_callbacks = Rc::new(ServoWindowCallbacks::new(
        callbacks,
        RefCell::new(init_opts.coordinates),
    ));

    let servo_builder = ServoBuilder::new(rendering_context.clone())
        .opts(opts)
        .preferences(preferences)
        .event_loop_waker(waker);

    #[cfg(feature = "webxr")]
    let servo_builder = servo_builder.webxr_registry(Box::new(XrDiscoveryWebXrRegistry::new(
        init_opts.xr_discovery,
    )));

    APP.with(|app| {
        let app_state = RunningAppState::new(
            init_opts.url,
            init_opts.density,
            rendering_context,
            servo_builder.build(),
            window_callbacks,
            servoshell_preferences,
        );
        *app.borrow_mut() = Some(app_state);
    });

    Ok(())
}

pub fn deinit() {
    APP.with(|app| {
        let app = app.replace(None).unwrap();
        if let Some(app_state) = Rc::into_inner(app) {
            app_state.deinit()
        }
    });
}
