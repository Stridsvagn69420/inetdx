use std::net::Ipv4Addr;
use std::process::ExitCode;

mod config;
mod shared;
mod echo;
mod discard;
mod daytime;
mod qotd;
mod chargen;
mod time;
mod hostname;

fn main() -> ExitCode {
	let cfg = config::Config::load().unwrap_or_default();
	println!("{cfg}");
	println!();

	// Configure Server
	let addr = Ipv4Addr::new(0, 0, 0, 0);
	let port = 7;

	if let Err(err) = echo::echo_tcp(addr, port) {
		println!("Could not start server ({}): {err}", err.kind());
		ExitCode::FAILURE
	} else {
		ExitCode::SUCCESS
	}
}