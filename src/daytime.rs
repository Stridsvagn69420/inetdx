use std::net::TcpStream;
use std::net::Ipv4Addr;
use std::io::Write;
use std::io;
use chrono::Local;
use crate::shared::tcp_server;

/// Daytime TCP-Server
pub(crate) fn daytime_tcp(addr: Ipv4Addr, port: u16) -> io::Result<()> {
	tcp_server(addr, port, move |stream: &mut TcpStream| {
		// Write RFC 2822 timestamp
		stream.write_all(Local::now().to_rfc2822().as_bytes())?;
		Ok(())
	})
}