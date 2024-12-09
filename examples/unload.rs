use jelly_fpgautil as fpgautil;
use jelly_uidmng as uidmng;

fn main() {
    uidmng::change_user().unwrap();

    fpgautil::unload().unwrap();
}
