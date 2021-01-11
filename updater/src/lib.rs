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

//! Updater for Vast executables

#![warn(missing_docs)]

extern crate client_traits;
extern crate common_types;
extern crate vastabi;
extern crate vastabi_derive;
extern crate vastcore;
extern crate vastcore_sync as sync;
extern crate vast_types;
extern crate keccak_hash as hash;
extern crate vast_bytes as bytes;
extern crate vast_hash_fetch as hash_fetch;
extern crate vast_path;
extern crate vast_version as version;
extern crate parking_lot;
extern crate rand;
extern crate semver;
extern crate target_info;

#[macro_use]
extern crate vastabi_contract;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[cfg(test)]
extern crate tempdir;

#[cfg(test)]
#[macro_use]
extern crate matches;

mod updater;
mod types;
mod service;

pub use service::Service;
pub use types::{ReleaseInfo, OperationsInfo, CapState, VersionInfo, ReleaseTrack};
pub use updater::{Updater, UpdateFilter, UpdatePolicy};
