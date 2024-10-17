use std::net::TcpStream;
use std::net::IpAddr;
use std::io::Write;
use std::io;
use crate::shared::{tcp_server, udp_server, BUFFER_SIZE_UDP};

#[cfg(target_family = "unix")]
use std::fs;

#[cfg(not(target_family = "unix"))]
use std::env;

pub(crate) const NAME: &str = "Hostname";
pub(crate) const PORT: u16 = 42;

/// Hostname TCP-Server
pub(crate) fn hostname_tcp(addr: IpAddr, port: u16) -> io::Result<()> {
	tcp_server(addr, port, NAME, move |stream: &mut TcpStream| {
		// Write Hostname
		let nodename = hostname()?;
		stream.write_all(nodename.as_bytes())?;
		Ok(())
	})
}

/// Hostname UDP-Server
pub(crate) fn hostname_udp(addr: IpAddr, port: u16) -> io::Result<()> {
	udp_server(addr, port, NAME, |socket| {
		// Accept Datagram
		let mut buf = [0; BUFFER_SIZE_UDP];
		let (_, peer) = socket.recv_from(&mut buf)?;

		// Return Hostname
		let nodename = hostname()?;
		socket.send_to(nodename.as_bytes(), peer)?;
		Ok(())
	})
}

/// Get Hostname
fn hostname() -> io::Result<String> {
	#[cfg(target_family = "unix")]
	let nodename = fs::read_to_string("/etc/hostname")?;

	#[cfg(not(target_family = "unix"))]
	let Ok(nodename) = env::var("COMPUTERNAME") else {
		return Err(io::Error::new(io::ErrorKind::NotFound, "Node name not found"));
	};

	Ok(nodename)
}