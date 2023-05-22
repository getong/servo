# Copyright 2023 The Servo Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

import platform

from .base import Base
from .windows import Windows


def host_platform():
    os_type = platform.system().lower()
    if os_type == "linux":
        os_type = "unknown-linux-gnu"
    elif os_type == "darwin":
        os_type = "apple-darwin"
    elif os_type == "android":
        os_type = "linux-androideabi"
    elif os_type == "windows":
        os_type = "pc-windows-msvc"
    elif os_type == "freebsd":
        os_type = "unknown-freebsd"
    else:
        os_type = "unknown"
    return os_type


def host_triple():
    os_type = host_platform()
    cpu_type = platform.machine().lower()
    if cpu_type in ["i386", "i486", "i686", "i768", "x86"]:
        cpu_type = "i686"
    elif cpu_type in ["x86_64", "x86-64", "x64", "amd64"]:
        cpu_type = "x86_64"
    elif cpu_type == "arm":
        cpu_type = "arm"
    elif cpu_type == "aarch64":
        cpu_type = "aarch64"
    else:
        cpu_type = "unknown"
    return f"{cpu_type}-{os_type}"


def get():
    # We import the concrete platforms in if-statements here, because
    # each one might have platform-specific imports which might not
    # resolve on all platforms.
    # TODO(mrobinson): We should do this for Windows too, once we
    # stop relying on platform-specific code outside of this module.
    # pylint: disable=import-outside-toplevel
    if "windows-msvc" in host_triple():
        return Windows()
    if "linux-gnu" in host_triple():
        from .linux import Linux
        return Linux()
    if "apple-darwin" in host_triple():
        from .macos import MacOS
        return MacOS()
    return Base()