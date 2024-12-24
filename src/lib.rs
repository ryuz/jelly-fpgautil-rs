use jelly_uidmng::*;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::io::Write;
use std::result::Result;
use std::process::{Command, Stdio};


fn directory_exists(path: &str) -> bool {
    let path = Path::new(path);
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

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
    let fname = Path::new(bitstream)
        .file_name()
        .ok_or("Failed to extract filename")?
        .to_str()
        .ok_or("Invalid filename")?;
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

// register accelerator package
pub fn register_accel(
    accel_name: &str,
    bin_file: &str,
    dtbo_file: &str,
    json_file: Option<&str>,
    overwrite: bool,
) -> Result<(), Box<dyn Error>> {
    // acclel_path のディレクトリ存在チェック
    let acclel_path = format!("/lib/firmware/xilinx/{}", accel_name);
    if directory_exists(&acclel_path) {
        if !overwrite {
            return Err("Accel already exists".into());
        }
        let out = command_root("rm", ["-rf", &acclel_path])?;
        if !out.status.success() {
            return Err("Failed to remove existing accel".into());
        }
    }
    command_root("mkdir", ["-p", &acclel_path])?;

    let bin_fname = Path::new(bin_file)
        .file_name()
        .ok_or("Failed to extract filename")?
        .to_str()
        .ok_or("Invalid filename")?;
    let dtbo_fname = Path::new(dtbo_file)
        .file_name()
        .ok_or("Failed to extract filename")?
        .to_str()
        .ok_or("Invalid filename")?;

    command_root("cp", [bin_file, &format!("{}/{}", acclel_path, bin_fname)])?;
    command_root(
        "cp",
        [dtbo_file, &format!("{}/{}", acclel_path, dtbo_fname)],
    )?;

    if let Some(json_file) = json_file {
        command_root(
            "cp",
            [json_file, &format!("{}/shell.json", acclel_path)],
        )?;
    } else {
        let json_data = "{\n    \"shell_type\" : \"XRT_FLAT\",\n    \"num_slots\" : \"1\"\n}\n";
        write_root(
            &format!("{}/shell.json", acclel_path),
            &json_data.as_bytes().to_vec(),
        )?;
    }

    Ok(())
}

// register accelerator package
pub fn register_accel_with_vec(
    accel_name: &str,
    bin_fname: &str,
    bin: &[u8],
    dtbo_fname: &str,
    dtbo: &[u8],
    json: Option<&str>,
    overwrite: bool,
) -> Result<(), Box<dyn Error>> {
    // acclel_path のディレクトリ存在チェック
    let acclel_path = format!("/lib/firmware/xilinx/{}", accel_name);
    if directory_exists(&acclel_path) {
        if !overwrite {
            return Err("Accel already exists".into());
        }
        let out = command_root("rm", ["-rf", &acclel_path])?;
        if !out.status.success() {
            return Err("Failed to remove existing accel".into());
        }
    }
    command_root("mkdir", ["-p", &acclel_path])?;
    write_root(&format!("{}/{}", acclel_path, bin_fname), bin)?;
    write_root(&format!("{}/{}", acclel_path, dtbo_fname), dtbo)?;
    if let Some(json) = json {
        write_root(&format!("{}/shell.json", acclel_path), &json.as_bytes())?;
    } else {
        let json_data = "{\n    \"shell_type\" : \"XRT_FLAT\",\n    \"num_slots\" : \"1\"\n}\n";
        write_root(
            &format!("{}/shell.json", acclel_path),
            &json_data.as_bytes().to_vec(),
        )?;
    }

    Ok(())
}


pub fn unregister_accel(accel_name: &str) -> Result<(), Box<dyn Error>> {
    let acclel_path = format!("/lib/firmware/xilinx/{}", accel_name);
    command_root("rm", ["-rf", &acclel_path])?;
    Ok(())
}


pub fn dtc_with_str(dts: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut child = Command::new("dtc")
        .args(["-I", "dts", "-O", "dtb", "-o", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(dts.as_bytes())?;
    }
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err("Failed to execute dtc".into());
    }
    Ok(output.stdout)
}
