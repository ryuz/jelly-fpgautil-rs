use jelly_fpgautil as fpgautil;
use jelly_uidmng as uidmng;

fn main() {
    uidmng::change_user().unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <bin_file> <dtbo_file>", args[0]);
        std::process::exit(1);
    }
    let bin_file = &args[1];
    let dtbo_file = &args[2];
    fpgautil::register_accel("fpgautil", bin_file, dtbo_file, None, true).unwrap();
}
