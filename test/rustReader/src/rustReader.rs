 use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path; 
use std::env;

fn main() -> io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() <2 {return Err(io::Error::new(io::ErrorKind::InvalidInput, " drop a file ontop"));
	}
	let path = Path::new("output.txt");
	let file = File::open(&path)?;
	let reader = io::BufReader::new(file);
	for line in reader.lines() {
		let line = line?;
		println!("Hans {}", line);
	}
	Ok(())
}