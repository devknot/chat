use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	
	let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
	
	let input = stdin();
	
	let handle = thread::spawn(move || {
		loop {
			let mut buffer = String::new();
			stream.read_to_string(&mut buffer).unwrap();
			if buffer.len() == 0 {
				continue;
			}
			println!("{}", buffer);
		}
	});
	
	
	
	handle.join();
	
	Ok(())
}

