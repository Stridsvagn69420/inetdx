use std::io;
use std::net::Ipv4Addr;
use std::net::{TcpListener, TcpStream};
use std::thread;

// App Metadata and Constants
pub(crate) const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub(crate) const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");
pub(crate) const CONFIG_FILE: &str = "config.toml";
pub(crate) const QOTD_FILE: &str = "qotd.txt";

/// Shared internal buffer size
pub(crate) const BUFFER_SIZE: usize = 4096;

/// TCP-Server
/// 
/// A wrapper for creating a TCP-Server. It already deals with multi-threading and logging.
/// - `addr`: The [IPv4 address](Ipv4Addr) to bind to.
/// - `port`: The port to bind to.
/// - `f`: The service handle function. It is wrapped inside of [tcp_handle].
pub(crate) fn tcp_server<F>(addr: Ipv4Addr, port: u16, mut f: F) -> io::Result<()>
where
	F: FnMut(&mut TcpStream) -> io::Result<()> + Send + Copy + 'static
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
		id += 1;
		let mut connection = match listener.accept() {
			Ok(conn) => {
				println!("[{id}] Connected to {}", conn.1);
				conn
			},
			Err(err) => {
				println!("[{id}] TCP-Handshake failed ({}): {err}", err.kind());
				continue;
			},
		};

		thread::spawn(move || {
			match f(&mut connection.0) {
				Ok(_) => println!("[{id}] Connection closed."),
				Err(err) => println!("[{id}] {}: {err}", err.kind()),
			}
		});
	}
}