use std::error::Error;

use rppal::gpio::{Gpio, Trigger};
use rppal::system::DeviceInfo;
use std::process::Command;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_PIN: u8 = 3;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Testing button input on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_PIN)?.into_input_pullup();
    pin.set_interrupt(Trigger::FallingEdge)?;
    pin.poll_interrupt(true, None)?;
    let output = Command::new("shutdown")
        .args(["-h", "now"])
        // .spawn()
        .output()
        .expect("Couldn't run shutdown command");
    // println!("{}", std::str::from_utf8(&output.stdout).unwrap());
    // println!("{}", std::str::from_utf8(&output.stderr).unwrap());
    Ok(())
}
