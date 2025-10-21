use jelly_uidmng as uidmng;

fn main() {
    let output = uidmng::command("uname", ["-m"]).unwrap();
    match String::from_utf8_lossy(&output.stdout).as_ref() {
        "armv7l\n" => println!("This is armv7l machine"),
        "aarch64\n" => println!("This is aarch64 machine"),
        "x86_64\n" => println!("This is x86_64 machine"),
        _ => println!("This is not x86_64 machine"),
    }
}
