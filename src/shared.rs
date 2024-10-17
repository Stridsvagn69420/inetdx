use std::io;
use std::net::IpAddr;
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::thread;

// App Metadata and Constants
pub(crate) const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub(crate) const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");
pub(crate) const CONFIG_FILE: &str = "config.toml";

/// Common TCP buffer size
/// 
/// The default size for each TCP server's read buffer.
/// Size does not matter that much here since TCP uses streams instead of single packets like UDP.
pub(crate) const BUFFER_SIZE_TCP: usize = 4096;

/// Common UDP Buffer size
/// 
/// The default size for each UDP server's read buffer.
/// 
/// This buffer is at the MTU for non-loopback LANs, because non-loopback networks usually have an MTU of around 1500 Bytes.
/// If the service does read directly from loopback, then use **65507** (not 65535 due to practical limitations);
/// 
/// The reason why buffer size matter is that [every datagram is a one-time read](UdpSocket::recv_from)
/// and you cannot get the actual size without messing with heap memory.
pub(crate) const BUFFER_SIZE_UDP: usize = 1500;

/// TCP-Server
/// 
/// A wrapper for creating a TCP-Server. It already deals with multi-threading and logging.
/// - `addr`: The [IPv6 address](Ipv6Addr) to bind to.
/// - `port`: The port to bind to.
/// - `f`: The service handle function.
pub(crate) fn tcp_server<F>(addr: IpAddr, port: u16, name: &str, mut f: F) -> io::Result<()>
where
	F: FnMut(&mut TcpStream) -> io::Result<()> + Send + Copy + 'static
{
	// Create TCP Listener
	let listener = match TcpListener::bind((addr, port)) {
		Err(err) => return Err(err),
		Ok(conn) => {
			println!("{name} TCP-Server started: {addr}:{port}");
			conn
		}
	};
	// Track TCP-Streams
	let mut id: u128 = 0;

	// Listener Loop
	loop {
		id += 1;
		let mut connection = match listener.accept() {
			Ok(conn) => {
				println!("[Port {port}] ({id}) Connected to {}", conn.1);
				conn
			},
			Err(err) => {
				println!("[Port {port}] ({id}) TCP-Handshake failed ({}): {err}", err.kind());
				continue;
			},
		};

		thread::spawn(move || {
			match f(&mut connection.0) {
				Ok(_) => println!("[Port {port}] ({id}) Stream closed."),
				Err(err) => println!("[Port {port}] ({id}) {}: {err}", err.kind()),
			}
		});
	}
}

/// UDP-Server
/// 
/// A wrapper for creating a UDP-Server.
/// - `addr`: The [IPv6 address](Ipv6Addr) to bind to.
/// - `port`: The port to bind to.
/// - `f`: The service handle function. Due to technical difficulties, the function needs to log seperately.
pub(crate) fn udp_server<F>(addr: IpAddr, port: u16, name: &str, mut f: F) -> io::Result<()>
where
	F: FnMut(&mut UdpSocket) -> io::Result<()> + Send + Copy + 'static
{
	// Create UDP Socket
	let mut socket = match UdpSocket::bind((addr, port)) {
		Err(err) => return Err(err),
		Ok(sock) => {
			println!("{name} UDP-Server started: {addr}:{port}");
			sock
		}
	};

	// Constantly iterate over service function
	loop {
		if let Err(err) = f(&mut socket) {
			println!("[Port {port}] {}: {err}", err.kind())
		}
	}
}