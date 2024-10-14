use std::net::TcpStream;
use std::io;
use std::io::Write;

pub fn echo_handle(stream: &mut TcpStream, data: &[u8], _: &mut bool) -> io::Result<()> {
	stream.write_all(data)
}