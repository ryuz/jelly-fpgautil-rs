use jelly_fpgautil as fpgautil;
use jelly_uidmng as uidmng;

fn main() {
    uidmng::set_allow_sudo(true);

    fpgautil::start_remoteproc(0).unwrap();
}
