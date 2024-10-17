use std::net::TcpStream;
use std::net::IpAddr;
use std::io::Write;
use std::io;
use std::time::SystemTime;
use crate::shared::{tcp_server, udp_server, BUFFER_SIZE_UDP};

pub(crate) const NAME: &str = "Time";
pub(crate) const PORT: u16 = 37;

/// Time TCP-Server
pub(crate) fn time_tcp(addr: IpAddr, port: u16) -> io::Result<()> {
	tcp_server(addr, port, NAME, move |stream: &mut TcpStream| {
		// Send UNIX Timestamp
		stream.write_all(&unixtime())?;
		Ok(())
	})
}

/// Time UDP-Server
pub(crate) fn time_udp(addr: IpAddr, port: u16) -> io::Result<()> {
	udp_server(addr, port, NAME, |socket| {
		// Accept Datagram
		let mut buf = [0; BUFFER_SIZE_UDP];
		let (_, peer) = socket.recv_from(&mut buf)?;

		// Send UNIX Timestamp
		socket.send_to(&unixtime(), peer)?;
		Ok(())
	})
}

/// UNIX Time
/// 
/// UNIX Time as Big-Endian 64-bit unsigned integer
fn unixtime() -> [u8; (u64::BITS / 8) as usize] {
	SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs().to_be_bytes()
}