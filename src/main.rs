use std::net::Ipv4Addr;
use std::process::ExitCode;

mod shared;
mod config;
mod hostname;
mod echo;

fn main() -> ExitCode {
	let cfg = config::Config::load().unwrap_or_default();
	println!("{cfg}");
	println!();

	// Configure Server
	let addr = Ipv4Addr::new(0, 0, 0, 0);
	let port = 7;

	// Run server
	if let Err(err) = shared::tcp_server(addr, port, echo::echo_handle) {
		println!("Could not start server ({}): {err}", err.kind());
		ExitCode::FAILURE
	} else {
		ExitCode::SUCCESS
	}
}