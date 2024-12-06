use std::error::Error;
use std::result::Result;
use std::path::Path;
use jelly_uidmng::*;


pub fn unload() -> Result<(), Box<dyn Error>> {
//  let out = command_root("dfx-mgr-client", ["-remove"])?;
//  if !out.status.success() {
//      return Err("Failed to unload : dfx-mgr-client -remove".into());
//  }

    command_root("dfx-mgr-client", ["-remove"])?;
    command_root("rmdir", ["/configfs/device-tree/overlays/full"])?;

    Ok(())
}

pub fn load(accel_name: &str) -> Result<(), Box<dyn Error>> {
    command_root("dfx-mgr-client", ["-load", accel_name])?;
    Ok(())
}

pub fn load_bitstream(bitstream: &str) -> Result<(), Box<dyn Error>> {
    let fname = Path::new(bitstream).file_name().ok_or("Failed to extract filename")?
                .to_str().ok_or("Invalid filename")?;

//  let fname = match path.file_name() {
//      Some(name) => name.to_str().ok_or("Invalid filename")?,
//      None => return Err("Failed to extract filename".into()),
//  };
    let firmware_path = format!("/lib/firmware/{}", fname);
//  println!("{}", firmware_path);
//  println!("{}", bitstream);
    let out = command_root("cp", [bitstream, &firmware_path])?;
//    println!("{}", String::from_utf8_lossy(&out.stdout));
//    println!("{}", String::from_utf8_lossy(&out.stderr));
//    println!("{}", out.status);
    let load_cmd = format!("echo {} > /sys/class/fpga_manager/fpga0/firmware", fname);
    command_root("sh", ["-c", &load_cmd])?;

    Ok(())
}

