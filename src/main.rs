extern crate huioncfg;

use std::process;

fn main() {
    
/* if let Err(e) = huioncfg::has_xrand() {
    println!("Command not fund {}", e);
    process::exit(1);
} */

huioncfg::teste_aplit();
  
}



struct Device {
    pad: i32,
    stilus: i32,
}
struct Config {
    monitor: String,
    devide: Device,

}