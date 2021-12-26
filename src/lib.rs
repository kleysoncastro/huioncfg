use std::process::Command;
use std::error::Error;
struct Device {
    pad: i32,
    stilus: i32,
}
struct Config {
    monitor: String,
    devide: Device,

}

pub fn who_xrand() -> Result<(), Box<dyn Error>> {

    let mut xrand = Command::new("xrandr")
    .arg("--listactivemonitors")
    .status()?;
    

    Ok(())
}