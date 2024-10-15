use std::net::TcpStream;
use std::net::Ipv4Addr;
use std::io::Write;
use std::io;
use std::time::SystemTime;
use crate::shared::tcp_server;

/// Time TCP-Server
pub(crate) fn time_tcp(addr: Ipv4Addr, port: u16) -> io::Result<()> {
	tcp_server(addr, port, move |stream: &mut TcpStream| {
		// Send UNIX Time as Big-Endian u64
		let duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
		let data = duration.as_secs().to_be_bytes();
		stream.write_all(&data)?;
		Ok(())
	})
}