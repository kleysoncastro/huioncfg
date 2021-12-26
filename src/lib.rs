use std::process::Command;
use std::process::Stdio;
use std::error::Error;
struct Device {
    pad: i32,
    stilus: i32,
}
struct Config {
    monitor: String,
    devide: Device,

}

pub fn has_xrand() -> Result<(), Box<dyn Error>> {

    let mut xrandr = Command::new("xrandr")
    .stdout(Stdio::piped())
    .arg("--listactivemonitors")
    .output()?;
    
    let stdout = String::from_utf8(xrandr.stdout)?;
    
   
    Ok(())
}

pub fn has_xsetwacom() -> Result<(), Box<dyn Error>> {


    let xsetwacom = Command::new("xsetwacom")
    // Tell the OS to record the command's output
    .stdout(Stdio::piped())
    .arg("--list")
    .arg("devices")
    .output()?;

    let stdout = String::from_utf8(xsetwacom.stdout)?;


    Ok(())
}

pub fn teste_aplit() {

         let mut split = "0: +*LVDS-1 1600/345x900/194+0+0  LVDS-1".split(' ');  
        let mut vec = Vec::new();
        for s in split {
            vec.push(s);
           // println!("{}", s)
        } 

        println!("{}", vec[vec.len() - 1]); 
       // let vec = split.collect::<Vec<&str>>();
        // OR
        //let vec: Vec<&str> = split.collect();

       // let mut split = "0: +*LVDS-1 1600/345x900/194+0+0  LVDS-1";

 
        
     
}