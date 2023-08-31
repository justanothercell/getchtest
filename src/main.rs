use std::process::exit;
use getch::Getch;
	
fn main() {
    let mut getch = Getch::new();
    loop {
		let c = getch.getch().expect("could not getch char");
		println!("{:?} 0x{:X} {}", c as char, c, c);
		if c == b'\x03' { exit(0) }
	}
}