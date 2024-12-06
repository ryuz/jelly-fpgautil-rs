use jelly_uidmng as uidmng;
use jelly_fpgautil as fpgautil;

fn main() {
    uidmng::change_user().unwrap();

    fpgautil::unload().unwrap();
}
