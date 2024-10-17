use std::net::IpAddr;
use std::io::Read;
use std::io;
use crate::shared::{tcp_server, udp_server, BUFFER_SIZE_TCP, BUFFER_SIZE_UDP};

pub(crate) const NAME: &str = "Discard";
pub(crate) const PORT: u16 = 9;

/// Discard TCP-Server
pub(crate) fn discard_tcp(addr: IpAddr, port: u16) -> io::Result<()> {
	tcp_server(addr, port, NAME, |stream| {
		loop {
			// Read client data and ignore it like a boss
			let mut buf = [0; BUFFER_SIZE_TCP];
			let n = stream.read(&mut buf)?;
			if n == 0 {
				break;
			}
		}
		Ok(())
	})
}

/// Discard UDP-Server
pub(crate) fn discard_udp(addr: IpAddr, port: u16) -> io::Result<()> {
	udp_server(addr, port, NAME, |socket| {
		// Accept Datagram
		let mut buf = [0; BUFFER_SIZE_UDP];
		let (n, peer) = socket.recv_from(&mut buf)?;

		// Debug Code
		if cfg!(debug_assertions) {
			let data = String::from_utf8_lossy(&buf[..n]);
			println!("DEBUG [{NAME}] Received {n} Bytes from {peer}: {data}");
		}
		Ok(())
	})
}