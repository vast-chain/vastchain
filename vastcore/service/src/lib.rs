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

extern crate ansi_term;
extern crate common_types;
extern crate client_traits;
extern crate vastcore;
extern crate vastcore_blockchain as blockchain;
extern crate vastcore_io as io;
extern crate vastcore_private_tx;
extern crate vastcore_sync as sync;
extern crate vast_types;
extern crate kvdb;
extern crate spec;
extern crate snapshot;

#[macro_use]
extern crate log;
#[macro_use]
extern crate trace_time;

#[cfg(test)]
extern crate vastcore_db;
#[cfg(test)]
extern crate tempdir;

mod service;

#[cfg(test)]
extern crate kvdb_rocksdb;

pub use service::{ClientService, PrivateTxService};
