use std::net::Ipv4Addr;
use std::io::Read;
use std::io;
use crate::shared::{BUFFER_SIZE, tcp_server};

/// Discard TCP-Server
pub(crate) fn discard_tcp(addr: Ipv4Addr, port: u16) -> io::Result<()> {
	tcp_server(addr, port, |stream| {
		loop {
			// Read client data and ignore it like a boss
			let mut buf = [0; BUFFER_SIZE];
			let n = stream.read(&mut buf)?;
			if n == 0 {
				break;
			}
		}
		Ok(())
	})
}