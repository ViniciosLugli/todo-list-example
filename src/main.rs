use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
extern crate bcrypt;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod response;
mod server;
mod task;
mod user;

use server::Server;

fn handle_client(mut stream: TcpStream, server: &Server) {
	let mut buffer = [0; 1024];
	match stream.read(&mut buffer) {
		Ok(_) => {
			let request = str::from_utf8(&buffer).unwrap_or_default();
			debug!("Received request: {}", request);

			let response = server.handle_request(request);
			stream.write_all(response.as_bytes()).unwrap();
			stream.flush().unwrap();
			info!("Response sent for request");
		}
		Err(e) => {
			error!("Error reading from stream: {}", e);
		}
	}
}

fn main() {
	pretty_env_logger::init();
	let server = Server::new();
	let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

	info!("Server listening on 0.0.0.0:3000");

	for stream in listener.incoming() {
		let server = server.clone();
		match stream {
			Ok(stream) => {
				info!("New client connected: {}", stream.peer_addr().unwrap());
				handle_client(stream, &server);
			}
			Err(e) => {
				error!("Failed to accept client: {}", e);
			}
		}
	}
}
