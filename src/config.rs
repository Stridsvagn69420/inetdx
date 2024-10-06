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

impl Default for Basic {
	fn default() -> Self {
		Self { tcp: true, udp: true }
	}
}

impl fmt::Display for Basic {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.tcp {
			write!(f, "TCP")?;
		}
		if self.tcp && self.udp {
			write!(f, "+")?;
		}
		if self.udp {
			write!(f, "UDP")?;
		}
		Ok(())
	}
}

/// inetdx config
/// 
/// The very basic config for inetdx
#[derive(Deserialize, Default)]
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