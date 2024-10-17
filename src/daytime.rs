use std::net::TcpStream;
use std::net::IpAddr;
use std::io::Write;
use std::io;
use chrono::Local;
use crate::shared::{tcp_server, udp_server, BUFFER_SIZE_UDP};

pub(crate) const NAME: &str = "Daytime";
pub(crate) const PORT: u16 = 13;

/// Daytime TCP-Server
pub(crate) fn daytime_tcp(addr: IpAddr, port: u16) -> io::Result<()> {
	tcp_server(addr, port, NAME, move |stream: &mut TcpStream| {
		// Write RFC 2822 timestamp
		let time = timestamp();
		stream.write_all(time.as_bytes())?;
		Ok(())
	})
}

/// Daytime UDP-Server
pub(crate) fn daytime_udp(addr: IpAddr, port: u16) -> io::Result<()> {
	udp_server(addr, port, NAME, |socket| {
		// Accept Datagram
		let mut buf = [0; BUFFER_SIZE_UDP];
		let (_, peer) = socket.recv_from(&mut buf)?;

		// Return RFCC 2822 Timestamp
		let time = timestamp();
		socket.send_to(time.as_bytes(), peer)?;
		Ok(())
	})
}

/// Get RFC 2822 Timestamp
fn timestamp () -> String {
	Local::now().to_rfc2822()
}