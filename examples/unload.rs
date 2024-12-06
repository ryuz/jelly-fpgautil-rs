use jelly_fpgautil as fpgautil;


fn main() {
    fpgautil::unload().unwrap();
    fpgautil::load("k26-starter-kits").unwrap();
    fpgautil::unload().unwrap();
    fpgautil::load("vadd").unwrap();
    fpgautil::unload().unwrap();
}
