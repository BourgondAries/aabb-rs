extern crate toml;

fn get_file_contents(filename: &str) -> std::io::Result<String> {
	use std::fs::File;
	use std::io::Read;
	let mut file = try!(File::open(filename));
	let mut string = String::new();
	try!(file.read_to_string(&mut string));
	Ok(string)
}

fn main() {
	use toml::Value;
	let y = &String::from("ok");
	let mut x = &String::from("what");
	x = y;
	match *x {
		ref y => println!("{}", y),
	}
	// use livetoml::livetoml::Interpret;
	let string = get_file_contents("test.toml").unwrap();
	let mut table: Value = string.parse()
		.unwrap();
	// table.eval("name.0 = derp").expect("");
	println!("{:?}", table.lookup("name.0"));
	println!("{:?}", table);
}
