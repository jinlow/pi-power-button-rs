use std::error::Error;

use rppal::gpio::{Gpio, Trigger};
use rppal::system::DeviceInfo;
use std::process::Command;

const GPIO_PIN: u8 = 3;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Testing button input on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_PIN)?.into_input_pullup();
    pin.set_interrupt(Trigger::FallingEdge)?;
    pin.poll_interrupt(true, None)?;
    let output = Command::new("shutdown")
        .args(["-h", "now"])
        .output()
        .expect("Couldn't run shutdown command");
    Ok(())
}
