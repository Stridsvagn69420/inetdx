use std::net::TcpStream;
use std::net::Ipv4Addr;
use std::io::Write;
use std::io;
use crate::shared::tcp_server;

#[cfg(target_family = "unix")]
use std::fs;

#[cfg(not(target_family = "unix"))]
use std::env;

/// Hostname TCP-Server
pub(crate) fn hostname_tcp(addr: Ipv4Addr, port: u16) -> io::Result<()> {
	tcp_server(addr, port, move |stream: &mut TcpStream| {
		#[cfg(target_family = "unix")]
		let nodename = fs::read_to_string("/etc/hostname")?;

		#[cfg(not(target_family = "unix"))]
		let Ok(nodename) = env::var("COMPUTERNAME") else {
			return Err(io::Error::new(io::ErrorKind::NotFound, "Node name not found"));
		};

		// Write hostname
		stream.write_all(nodename.as_bytes())?;
		Ok(())
	})
}