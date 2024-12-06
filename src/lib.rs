use std::error::Error;
use std::result::Result;
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

