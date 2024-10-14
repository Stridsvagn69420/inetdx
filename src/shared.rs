use std::io;
use std::io::Read;
use std::net::Ipv4Addr;

use std::net::{TcpListener, TcpStream};
//use tokio::net::UdpSocket;
use std::thread;

// App Metadata
pub(crate) const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub(crate) const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

// File constants
pub(crate) const CONFIG_FILE: &str = "config.toml";
pub(crate) const QOTD_FILE: &str = "qotd.txt";


/// Internal buffer size
const BUFFER_SIZE: usize = 4096;

/// TCP-Server
/// 
/// A wrapper for creating a TCP-Server. It already deals with multi-threading and logging.
/// - `addr`: The [IPv4 address](Ipv4Addr) to bind to.
/// - `port`: The port to bind to.
/// - `f`: The service handle function. It is wrapped inside of [tcp_handle].
pub(crate) fn tcp_server<F>(addr: Ipv4Addr, port: u16, f: F) -> io::Result<()>
where
	F: FnMut(&mut TcpStream, &[u8], &mut bool) -> io::Result<()> + Send + Copy + 'static
{
	let listener = match TcpListener::bind((addr, port)) {
		Err(err) => return Err(err),
		Ok(conn) => {
			println!("Started server at {addr} on Port {port}");
			conn
		}
	};

	// Track connections
	let mut id: u128 = 0;
	loop {
		let connection = match listener.accept() {
			Ok(conn) => {
				id += 1;
				println!("[{id}] Connected to {}", conn.1);
				conn
			},
			Err(err) => {
				println!("[ERROR] TCP-Handshake failed ({}): {err}", err.kind());
				continue;
			},
		};

		thread::spawn(move || {
			match tcp_handle(connection.0, f) {
				Ok(_) => println!("[{id}] Connection closed."),
				Err(err) => println!("[{id}] {}: {err}", err.kind()),
			};
		});
	}
}

/// TCP-Handle Wrapper
/// 
/// A wrapper that aleady reads the client's data and creates a loop.
/// - `f`: The TCP service logic function.
fn tcp_handle<F>(mut stream: TcpStream, mut f: F) -> io::Result<()> 
where
	F: FnMut(&mut TcpStream, &[u8], &mut bool) -> io::Result<()>
{
	loop {
		// Read client data
		let mut buf = [0; BUFFER_SIZE];
		let n = stream.read(&mut buf)?;
		if n == 0 {
			break;
		}

		// Pass to external handle and check for one-shot service
		let mut oneshot = false;
		f(&mut stream, &buf, &mut oneshot)?;
		if oneshot {
			break;
		}
	}
	Ok(())
}