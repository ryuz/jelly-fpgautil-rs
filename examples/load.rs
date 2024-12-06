use jelly_fpgautil as fpgautil;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <accel_name>", args[0]);
        std::process::exit(1);
    }
    let accel_name = &args[1];
    fpgautil::load(accel_name).unwrap();
}
