use std::net::Ipv4Addr;
use std::io::{Read, Write};
use std::io;
use crate::shared::{BUFFER_SIZE, tcp_server};

/// Echo TCP-Server
pub(crate) fn echo_tcp(addr: Ipv4Addr, port: u16) -> io::Result<()> {
	tcp_server(addr, port, |stream| {
		loop {
			// Read client data and write it back
			let mut buf = [0; BUFFER_SIZE];
			let n = stream.read(&mut buf)?;
			if n == 0 {
				break;
			}
			stream.write_all(&buf)?;
		}
		Ok(())
	})
}