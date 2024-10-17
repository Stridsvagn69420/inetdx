use crate::shared::{APP_NAME, CONFIG_FILE};

use std::default::Default;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use apputils::config::Cfg;
use serde::Deserialize;
use toml::from_str;

/// Basic Service Config
/// 
/// A simple config struct to toggle TCP and UDP
#[derive(Deserialize, Clone, Copy)]
pub(crate) struct Basic {
	pub tcp: bool,
	pub udp: bool,
	pub port: u16
}

impl Basic {
	/// Disabled
	/// 
	/// Creates dual-bool basic config with both TCP and UDP deactivated.
	/// Use the [Default]-trait implementation, if you want TCP and UDP activated.
	pub fn disabled() -> Self {
		Self { tcp: false, udp: false, port: 0 }
	}
}

impl Default for Basic {
	fn default() -> Self {
		Self { tcp: true, udp: true, port: 0 }
	}
}

impl fmt::Display for Basic {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.tcp && !self.udp{
			write!(f, "{} TCP", self.port)
		} else if self.tcp && self.udp {
			write!(f, "{} TCP+UDP", self.port)
		} else if self.udp && !self.tcp {
			write!(f, "{} UDP", self.port)
		} else {
			write!(f, "disabled")
		}
	}
}

/// Listener Address Config
/// 
/// Configures the listener address used
#[derive(Deserialize, Clone, Copy, Default)]
pub(crate) struct Listener {
	/// Use IPv6
	///
	/// Determines if IPv6 will be used or IPv4.
	/// Does not matter on dual-stack systems, but on e.g. Windows.
	pub ipv6: bool,

	/// Localhost only
	///
	/// Determines if it should only bind to the loopback network or to any connected network.
	pub localhost: bool
}

impl From<&Listener> for IpAddr {
	fn from(val: &Listener) -> Self {
		if val.ipv6 {
			if val.localhost {
				IpAddr::V6(Ipv6Addr::LOCALHOST)
			} else {
				IpAddr::V6(Ipv6Addr::UNSPECIFIED)
			}
		} else if val.localhost {
			IpAddr::V4(Ipv4Addr::LOCALHOST)
		} else {
			IpAddr::V4(Ipv4Addr::UNSPECIFIED)
		}
	}
}

impl From<Listener> for IpAddr {
	fn from(val: Listener) -> Self {
		Self::from(&val)
	}
}

impl From<Config> for Listener {
	fn from(value: Config) -> Self {
		value.listener
	}
}

impl From<&Config> for Listener {
	fn from(value: &Config) -> Self {
		value.listener
	}
}

impl fmt::Display for Listener {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", IpAddr::from(self))
	}
}

/// inetdx master config
/// 
/// The top-level config for inetdx
#[derive(Deserialize, Clone, Copy)]
pub(crate) struct Config {
	listener: Listener,
	pub echo: Basic,
	pub discard: Basic,
	pub daytime: Basic,
	pub qotd: Basic,
	pub chargen: Basic,
	pub time: Basic,
	pub hostname: Basic
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

impl From<Config> for IpAddr {
	fn from(val: Config) -> Self {
		Self::from(val.listener)
	}
}

impl From<&Config> for IpAddr {
	fn from(val: &Config) -> Self {
		Self::from(val.listener)
	}
}

impl Default for Config {
	fn default() -> Self {
		let mut config = Self {
			listener: Default::default(),
			echo: Default::default(),
			discard: Default::default(),
			daytime: Default::default(),
			qotd: Default::default(),
			chargen: Basic::disabled(),
			time: Default::default(),
			hostname: Default::default()
		};
		
		// Set default service ports
		config.echo.port = 7;
		config.discard.port = 9;
		config.daytime.port = 13;
		config.qotd.port = 17;
		config.chargen.port = 19;
		config.time.port = 37;
		config.hostname.port = 42;

		config
	}
}

impl fmt::Display for Config {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "Listener address: {}", self.listener)?;
		writeln!(f, "Echo: {}", self.echo)?;
		writeln!(f, "Discard: {} ", self.discard)?;
		writeln!(f, "Daytime: {} ", self.daytime)?;
		writeln!(f, "QotD: {} ", self.qotd)?;
		writeln!(f, "Chargen: {} ", self.chargen)?;
		writeln!(f, "Time: {}", self.time)?;
		write!(f, "Hostname: {}", self.hostname)
	}
}