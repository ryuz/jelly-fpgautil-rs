use jelly_fpgautil as fpgautil;
use jelly_uidmng as uidmng;

fn main() {
    uidmng::change_user().unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <accel_name>", args[0]);
        std::process::exit(1);
    }
    let accel_name = &args[1];
    fpgautil::load(accel_name).unwrap();
}
