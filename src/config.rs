use crate::meta::{APP_NAME, CONFIG_FILE};

use std::default::Default;
use std::fmt;

use apputils::config::Cfg;
use serde::Deserialize;
use toml::from_str;

/// Basic Service Config
/// 
/// A simple config struct to toggle TCP and UDP
#[derive(Deserialize)]
pub(crate) struct Basic {
	pub tcp: bool,
	pub udp: bool
}

impl Basic {
	/// Disabled
	/// 
	/// Creates dual-bool basic config with both TCP and UDP deactivated.
	/// Use the [Default]-trait implementation, if you want TCP and UDP activated.
	pub fn disabled() -> Self {
		Self { tcp: false, udp: false }
	}
}

impl Default for Basic {
	fn default() -> Self {
		Self { tcp: true, udp: true }
	}
}

impl fmt::Display for Basic {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.tcp && !self.udp{
			write!(f, "TCP")
		} else if self.tcp && self.udp {
			write!(f, "TCP+UDP")
		} else if self.udp && !self.tcp {
			write!(f, "UDP")
		} else {
			write!(f, "disabled")
		}
	}
}

/// inetdx config
/// 
/// The very basic config for inetdx
#[derive(Deserialize)]
pub(crate) struct Config {
	pub echo: Basic,
	pub discard: Basic,
	pub daytime: Basic,
	pub qotd: Basic,
	pub chargen: Basic,
	pub time: Basic
}

impl Config {
	/// Load and Parse Config
	/// 
	/// Loads the global config and parses it via [toml]
	pub fn load() -> Option<Self> {
		let tomltxt = Cfg::global_read(APP_NAME, CONFIG_FILE).ok()?;
		from_str(&tomltxt).ok()
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			echo: Default::default(),
			discard: Default::default(),
			daytime: Default::default(),
			qotd: Default::default(),
			chargen: Basic::disabled(),
			time: Default::default()
		}
	}
}

impl fmt::Display for Config {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "Echo: {}", self.echo)?;
		writeln!(f, "Discard: {} ", self.discard)?;
		writeln!(f, "Daytime: {} ", self.daytime)?;
		writeln!(f, "QotD: {} ", self.qotd)?;
		writeln!(f, "Chargen: {} ", self.chargen)?;
		write!(f, "Time: {}", self.time)
	}
}