// META: script=/resources/testdriver.js?feature=bidi
// META: script=/resources/testdriver-vendor.js
// META: script=/bluetooth/resources/bluetooth-test.js
// META: script=/bluetooth/resources/bluetooth-fake-devices.js
// META: timeout=long
const test_desc =
    'A device with name and no UUIDs nearby. Should be found if ' +
    'acceptAllDevices is true.';
const name = 'LE Device';

bluetooth_bidi_test(async () => {
  let {device} = await setUpPreconnectedFakeDevice({
    fakeDeviceOptions: {name: name},
    requestDeviceOptions: {acceptAllDevices: true}
  });
  assert_equals(device.name, name);
}, test_desc);
