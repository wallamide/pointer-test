#![no_main]
#![no_std]
#![feature(macro_metavar_expr)]
#![feature(type_alias_impl_trait)]
#![feature(generic_const_exprs)]

// TODO: Edit the `definition.json` file in the `src` folder to match your keyboard.
// _generated.rs is generated at build time, and will contain a serialized version of your Vial definition file to be compiled into your firmware.
// This file won't be generated if you don't specify the "vial" feature flag for rumcake.
#[cfg(vial)]
include!(concat!(env!("OUT_DIR"), "/_generated.rs"));

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyberon;
use rumcake::keyboard;

#[keyboard(usb, pointer(driver_setup_fn = test_pointer,))]
pub struct NuPointer;

impl rumcake::pointer::PointingDevice for NuPointer {}
struct MyIQS5xx;
impl rumcake::drives::iqs5xx::IQS5xxPointerDriver for MyIQS5xx {}
async fn test_pointer() -> impl rumcake::pointer::PointingDriver {
    rumcake::drivers::iqs5xx::setup_driver(
        rumcake::drivers::iqs5xx::setup_i2c! {
            interrupt: P0_22,
            i2c: P0_24,
            sda: P0_17,
            scl: P0_20,
        },
        input_pin!(i2c),
        output_pin!(interrupt),
        MyIQS5xx,
    )
}

// Basic keyboard configuration
use rumcake::keyboard::Keyboard;
impl Keyboard for NuPointer {
    const MANUFACTURER: &'static str = "wallamide";
    const PRODUCT: &'static str = "nupointer";
    const SERIAL_NUMBER: &'static str = "0420";
}

// USB configuration
// using PID/VID from https://pid.codes/1209/
use rumcake::usb::USBKeyboard;
impl USBKeyboard for NuPointer {
    const USB_VID: u16 = 0x1209;
    const USB_PID: u16 = 0x000F; // TODO: this is a test pid, i'll use new PID/get one from pid.codes
}
