use jelly_uidmng as uidmng;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::result::Result;

fn directory_exists(path: &str) -> bool {
    let path = Path::new(path);
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

fn get_fname(path: &str) -> Result<&str, Box<dyn Error>> {
    let fname = Path::new(path)
        .file_name()
        .ok_or("Failed to extract filename")?
        .to_str()
        .ok_or("Invalid filename")?;
    Ok(fname)
}

pub fn set_allow_sudo(allow: bool) {
    uidmng::set_allow_sudo(allow);
}

pub fn unload(slot: i32) -> Result<(), Box<dyn Error>> {
    let _ = uidmng::command_root("dfx-mgr-client", ["-remove", &slot.to_string()]);
    let _ = uidmng::command_root("rmdir", ["/configfs/device-tree/overlays/full"]);
    let _ = uidmng::write_root("/sys/class/fpga_manager/fpga0/flags", b"0");
    Ok(())
}

pub fn load(accel_name: &str) -> Result<i32, Box<dyn Error>> {
    uidmng::command_root("dfx-mgr-client", ["-load", accel_name])?;
    Ok(0)
}

pub fn copy_to_firmware(path: &str) -> Result<(), Box<dyn Error>> {
    let fname = get_fname(path)?;
    let firmware_path = format!("/lib/firmware/{}", fname);
    let out = uidmng::command_root("cp", [path, &firmware_path])?;
    if !out.status.success() {
        return Err(format!("Faile to copy {} to firmware", path).into());
    }
    Ok(())
}

pub fn write_to_firmware(name: &str, bin: &[u8]) -> Result<(), Box<dyn Error>> {
    let firmware_path = format!("/lib/firmware/{}", name);
    uidmng::write_root(&firmware_path, bin)
}

pub fn remove_firmware(name: &str) -> Result<(), Box<dyn Error>> {
    let firmware_path = format!("/lib/firmware/{}", name);
    uidmng::command_root("rm", ["-f", &firmware_path])?;
    Ok(())
}

pub fn load_bitstream_from_firmware(bitstream_name: &str) -> Result<(), Box<dyn Error>> {
    let load_cmd = format!(
        "echo {} > /sys/class/fpga_manager/fpga0/firmware",
        bitstream_name
    );
    let output = uidmng::command_root("sh", ["-c", &load_cmd])?;
    if !output.status.success() {
        return Err("Failed to load bitstream".into());
    }
    Ok(())
}

pub fn load_bitstream(bitstream_path: &str) -> Result<(), Box<dyn Error>> {
    let fname = get_fname(bitstream_path)?;
    copy_to_firmware(bitstream_path)?;
    load_bitstream_from_firmware(fname)?;
    Ok(())
}

pub fn load_bitstream_with_vec(bitstream_vec: &[u8]) -> Result<(), Box<dyn Error>> {
    uidmng::write_root("/lib/firmware/jelly-fpgautil.bin", bitstream_vec)?;
    uidmng::write_root(
        "/sys/class/fpga_manager/fpga0/firmware",
        b"jelly-fpgautil.bin",
    )?;
    Ok(())
}

pub fn load_dtbo_from_firmware(dtb_name: &str) -> Result<(), Box<dyn Error>> {
    uidmng::write_root("/sys/class/fpga_manager/fpga0/flags", b"0")?;
    let output = uidmng::command_root("mkdir", ["/configfs/device-tree/overlays/full"])?;
    if !output.status.success() {
        return Err("Failed to mkdir /configfs/device-tree/overlays/full".into());
    }
    uidmng::write_root(
        "/configfs/device-tree/overlays/full/path",
        dtb_name.as_bytes(),
    )?;

    for _ in 0..10 {
        if uidmng::read("/configfs/device-tree/overlays/full/status")? == b"applied\n" {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_micros(100));
    }
    return Err("Timeout to apply dtbo".into());
}

pub fn load_dtbo(dtb_path: &str) -> Result<(), Box<dyn Error>> {
    let fname = get_fname(dtb_path)?;
    copy_to_firmware(dtb_path)?;
    load_dtbo_from_firmware(fname)?;
    Ok(())
}

pub fn load_dtb_with_vec(dtb_path: &[u8]) -> Result<(), Box<dyn Error>> {
    uidmng::write_root("/lib/firmware/jelly-fpgautil.dtbo", dtb_path)?;
    load_dtbo_from_firmware("jelly-fpgautil.dtbo")
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
        let out = uidmng::command_root("rm", ["-rf", &acclel_path])?;
        if !out.status.success() {
            return Err("Failed to remove existing accel".into());
        }
    }
    uidmng::command_root("mkdir", ["-p", &acclel_path])?;

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

    uidmng::command_root("cp", [bin_file, &format!("{}/{}", acclel_path, bin_fname)])?;
    uidmng::command_root(
        "cp",
        [dtbo_file, &format!("{}/{}", acclel_path, dtbo_fname)],
    )?;

    if let Some(json_file) = json_file {
        uidmng::command_root("cp", [json_file, &format!("{}/shell.json", acclel_path)])?;
    } else {
        let json_data = "{\n    \"shell_type\" : \"XRT_FLAT\",\n    \"num_slots\" : \"1\"\n}\n";
        uidmng::write_root(
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
        let out = uidmng::command_root("rm", ["-rf", &acclel_path])?;
        if !out.status.success() {
            return Err("Failed to remove existing accel".into());
        }
    }
    uidmng::command_root("mkdir", ["-p", &acclel_path])?;
    uidmng::write_root(&format!("{}/{}", acclel_path, bin_fname), bin)?;
    uidmng::write_root(&format!("{}/{}", acclel_path, dtbo_fname), dtbo)?;
    if let Some(json) = json {
        uidmng::write_root(&format!("{}/shell.json", acclel_path), &json.as_bytes())?;
    } else {
        let json_data = "{\n    \"shell_type\" : \"XRT_FLAT\",\n    \"num_slots\" : \"1\"\n}\n";
        uidmng::write_root(
            &format!("{}/shell.json", acclel_path),
            &json_data.as_bytes().to_vec(),
        )?;
    }

    Ok(())
}

pub fn unregister_accel(accel_name: &str) -> Result<(), Box<dyn Error>> {
    let acclel_path = format!("/lib/firmware/xilinx/{}", accel_name);
    uidmng::command_root("rm", ["-rf", &acclel_path])?;
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

pub fn xlnx_bitstream_to_bin(bit_path: &str, bin_path: &str, arch: &str) -> Result<(), Box<dyn Error>> {
    let mut bif_file = tempfile::Builder::new().suffix(".bif").tempfile()?;
    bif_file.write(format!("all:\n{{\n    {}\n}}\n", bit_path).as_bytes())?;
    let bif_path = bif_file.path().to_path_buf();
    let output = uidmng::command(
        "bootgen",
        [
            "-w",
            "-image",
            &bif_path.to_str().unwrap(),
            "-arch",
            arch,
//            "-process_bitstream",
//            "bin",
            "-o",
            bin_path,
        ],
    )?;
    if !output.status.success() {
        return Err("Failed to execute bootgen".into());
    }
    Ok(())
}


pub fn xlnx_bitstream_to_bin_with_vec(bitstream: &[u8], arch: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bit_file = tempfile::Builder::new().suffix(".bit").tempfile()?;
    let     bin_file = tempfile::Builder::new().suffix(".bin").tempfile()?;
    bit_file.write(bitstream)?;
    let bit_path = bit_file.path().to_str().unwrap();
    let bin_path = bin_file.path().to_str().unwrap();
    xlnx_bitstream_to_bin(bit_path, bin_path, arch)?;
    let bin = std::fs::read(bin_path)?;
    Ok(bin)
}
