use bcrypt::{hash, verify};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[cfg(feature = "async")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async")]
use tokio::net::{TcpListener, TcpStream};

#[cfg(feature = "sync")]
use std::io::{Read, Write};
#[cfg(feature = "sync")]
use std::net::{TcpListener, TcpStream};

mod response;
mod server;
mod task;
mod user;

use server::Server;

#[cfg(feature = "sync")]
fn handle_client(mut stream: TcpStream, server: &Server) {
	let mut buffer = [0; 1024];
	match stream.read(&mut buffer) {
		Ok(_) => {
			let request = std::str::from_utf8(&buffer).unwrap_or_default();
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

#[cfg(feature = "async")]
async fn handle_client(mut stream: TcpStream, server: Arc<Server>) {
	let mut buffer = vec![0; 1024];
	while let Ok(n) = stream.read(&mut buffer).await {
		if n == 0 {
			break;
		}

		let request = std::str::from_utf8(&buffer[..n]).unwrap_or_default();
		debug!("Received request: {}", request);

		let response = server.handle_request(request);
		stream.write_all(response.as_bytes()).await.unwrap();
		stream.flush().await.unwrap();
		info!("Response sent for request");
	}
}

#[cfg(feature = "sync")]
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

#[cfg(feature = "async")]
#[tokio::main]
async fn main() {
	pretty_env_logger::init();
	let server = Arc::new(Server::new());
	let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

	info!("Server listening on 0.0.0.0:3000");

	loop {
		let (stream, addr) = listener.accept().await.unwrap();
		let server = server.clone();
		info!("New client connected: {}", addr);
		tokio::spawn(async move {
			handle_client(stream, server).await;
		});
	}
}
