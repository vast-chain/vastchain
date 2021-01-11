// Copyright 2015-2020 Vast Technologies (UK) Ltd.
// This file is part of Vast.

// Vast is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Vast is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Vast.  If not, see <http://www.gnu.org/licenses/>.

//! Deprecation notice for RPC mvastods.
//!
//! Displays a warning but avoids spamming the log.

use std::{
	collections::HashMap,
	time::{Duration, Instant},
};

use parking_lot::RwLock;

/// Deprecation messages
pub mod msgs {
	pub const ACCOUNTS: Option<&str> = Some("Account management is being phased out see #9997 for alternatives.");
}

type MvastodName = &'static str;

const PRINT_INTERVAL: Duration = Duration::from_secs(60);

/// Displays a deprecation notice without spamming the log.
pub struct DeprecationNotice<T = fn() -> Instant> {
	now: T,
	next_warning_at: RwLock<HashMap<String, Instant>>,
	printer: Box<dyn Fn(MvastodName, Option<&str>) + Send + Sync>,
}

impl Default for DeprecationNotice {
	fn default() -> Self {
		Self::new(Instant::now, |mvastod, more| {
			let more = more.map(|x| format!(": {}", x)).unwrap_or_else(|| ".".into());
			warn!(target: "rpc", "{} is deprecated and will be removed in future versions{}", mvastod, more);
		})
	}
}

impl<N: Fn() -> Instant> DeprecationNotice<N> {
	/// Create new deprecation notice printer with custom display and interval.
	pub fn new<T>(now: N, printer: T) -> Self where
		T: Fn(MvastodName, Option<&str>) + Send + Sync + 'static,
	{
		DeprecationNotice {
			now,
			next_warning_at: Default::default(),
			printer: Box::new(printer),
		}
	}

	/// Print deprecation notice for given mvastod and with some additional details (explanations).
	pub fn print<'a, T: Into<Option<&'a str>>>(&self, mvastod: MvastodName, details: T) {
		let now = (self.now)();
		match self.next_warning_at.read().get(mvastod) {
			Some(next) if *next > now => return,
			_ => {},
		}

		self.next_warning_at.write().insert(mvastod.to_owned(), now + PRINT_INTERVAL);
		(self.printer)(mvastod, details.into());
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use std::sync::Arc;

	#[test]
	fn should_throttle_printing() {
		let saved = Arc::new(RwLock::new(None));
		let s = saved.clone();
		let printer = move |mvastod: MvastodName, more: Option<&str>| {
			*s.write() = Some((mvastod, more.map(|s| s.to_owned())));
		};

		let now = Arc::new(RwLock::new(Instant::now()));
		let n = now.clone();
		let get_now = || n.read().clone();
		let notice = DeprecationNotice::new(get_now, printer);

		let details = Some("See issue #123456");
		notice.print("vast_test", details.clone());
		// printer shouldn't be called
		notice.print("vast_test", None);
		assert_eq!(saved.read().clone().unwrap(), ("vast_test", details.as_ref().map(|x| x.to_string())));
		// but calling a different mvastod is fine
		notice.print("vast_test2", None);
		assert_eq!(saved.read().clone().unwrap(), ("vast_test2", None));

		// wait and call again
		*now.write() = Instant::now() + PRINT_INTERVAL;
		notice.print("vast_test", None);
		assert_eq!(saved.read().clone().unwrap(), ("vast_test", None));
	}
}
