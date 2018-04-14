extern crate qrcode;
extern crate image;

use qrcode::QrCode;
use image::Luma;
use std::io::Write;

fn main() {
	//the first argument is the data and it must be supplied, second argument is optional output path
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2  || args[1].eq(&"-h".to_string()){
		writeln!(std::io::stderr(), "QR image generator").unwrap();
		writeln!(std::io::stderr(), "Usage: qr INPUT [OUTPUT]").unwrap();
		writeln!(std::io::stderr(), "Example: qr https://www.google.com ./qr_to_google.png").unwrap();
		std::process::exit(1);
	}
	// Encode some data into bits.
	let code = QrCode::new(&args[1]).expect("error parsing input argument");
	// Render the bits into an image.
	let image = code.render::<Luma<u8>>().build();
	if args.len() < 3 {
		image.save("./qr.png").unwrap();
	}
	image.save(&args[2]).expect("error parsing output argument")
}