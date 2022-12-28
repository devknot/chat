use std::net::{TcpListener, TcpStream};
use std::io::Write;

struct Usuários {
	pub quantidade: u8,
}

fn handle_client(mut stream: TcpStream) {
	let mut buffer = String::from("versão 0.0.1");
	stream.write(buffer.as_bytes()).unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	
	for stream in listener.incoming() {
		 handle_client(stream.unwrap());
	}
	Ok(())
}

