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
	// TODO: Add main logic
	let cfg = config::Config::load().unwrap_or_default();
	println!("{cfg}");
	println!();

	// Configure example Server
	if let Err(err) = echo::echo_udp(cfg.into(), echo::PORT) {
		println!("Could not start server ({}): {err}", err.kind());
		ExitCode::FAILURE
	} else {
		ExitCode::SUCCESS
	}
}