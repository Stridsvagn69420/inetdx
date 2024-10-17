use std::net::IpAddr;
use std::io::{Read, Write};
use std::io;
use crate::shared::{tcp_server, udp_server, BUFFER_SIZE_TCP};

pub(crate) const NAME: &str = "Echo";
pub(crate) const PORT: u16 = 7;

const BUFFER_SIZE_UDP: usize = 65507;

/// Echo TCP-Server
pub(crate) fn echo_tcp(addr: IpAddr, port: u16) -> io::Result<()> {
	tcp_server(addr, port, NAME, |stream| {
		loop {
			// Read client data and write it back
			let mut buf = [0; BUFFER_SIZE_TCP];
			let n = stream.read(&mut buf)?;
			if n == 0 {
				break;
			}
			stream.write_all(&buf[..n])?;
		}
		Ok(())
	})
}

/// Echo UDP-Server
pub(crate) fn echo_udp(addr: IpAddr, port: u16) -> io::Result<()> {
	udp_server(addr, port, NAME, |socket| {
		// Accept Datagram
		let mut buf = [0; BUFFER_SIZE_UDP];
		let (n, peer) = socket.recv_from(&mut buf)?;

		// Debug Code
		if cfg!(debug_assertions) {
			let data = String::from_utf8_lossy(&buf[..n]);
			println!("DEBUG [{NAME}] Received {n} Bytes from {peer}: {data}");
		}

		// Return if data was sent
		if n > 0 {
			socket.send_to(&buf[..n], peer)?;
		}
		Ok(())
	})
}