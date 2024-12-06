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
    let firmware_path = format!("/lib/firmware/{}", fname);
    let out = command_root("cp", [bitstream, &firmware_path])?;
    if !out.status.success() {
        return Err("Failed to copy bitstream".into());
    }
    let load_cmd = format!("echo {} > /sys/class/fpga_manager/fpga0/firmware", fname);
    let out = command_root("sh", ["-c", &load_cmd])?;
    if !out.status.success() {
        return Err("Failed to load bitstream".into());
    }

    Ok(())
}
