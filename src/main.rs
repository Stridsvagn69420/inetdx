mod meta;
mod config;

use config::Config;

fn main() {
	let conf = Config::load().unwrap_or_default();
	println!("{}", conf);
}