use std::process::Command;


fn main() {
	let output = Command::new("du")
		.args(["-sh", "/Users/lenix/Downloads/sysenix"])
		.output()
		.unwrap();

	println!("{}", String::from_utf8_lossy(&output.stdout));
}
