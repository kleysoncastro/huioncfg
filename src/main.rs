extern crate huioncfg;

use std::process;

fn main() {
    
if let Err(e) = huioncfg::who_xrand() {
    println!("Command not fund {}", e);
    process::exit(1);
}
  
}



struct Device {
    pad: i32,
    stilus: i32,
}
struct Config {
    monitor: String,
    devide: Device,

}