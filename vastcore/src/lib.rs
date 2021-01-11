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

#![warn(missing_docs, unused_extern_crates)]

//! Vastcore library

extern crate account_state;
extern crate ansi_term;
extern crate client_traits;
extern crate common_types as types;
extern crate engine;
extern crate vastcore_blockchain as blockchain;
extern crate vastcore_call_contract as call_contract;
extern crate vastcore_db as db;
extern crate vastcore_io as io;
extern crate vastcore_miner;
extern crate vast_types;
extern crate executive_state;
extern crate futures;
extern crate hash_db;
extern crate itertools;
extern crate journaldb;
extern crate keccak_hash as hash;
extern crate kvdb;
extern crate machine;
extern crate memory_cache;
extern crate vast_bytes as bytes;
extern crate parking_lot;
extern crate trie_db as trie;
extern crate patricia_trie_vast as vasttrie;
extern crate rand;
extern crate rayon;
extern crate registrar;
extern crate rlp;
extern crate rustc_hex;
extern crate serde;
extern crate snapshot;
extern crate spec;
extern crate state_db;
extern crate trace;
extern crate trie_vm_factories;
extern crate triehash_vast as triehash;
extern crate unexpected;
extern crate using_queue;
extern crate verification;
extern crate vm;

#[cfg(test)]
extern crate account_db;
#[cfg(test)]
extern crate vastcore_accounts as accounts;
#[cfg(test)]
extern crate stats;

#[cfg(feature = "stratum")]
extern crate vastcore_stratum;

#[cfg(feature = "stratum")]
extern crate vastash;

#[cfg(any(test, feature = "test-helpers"))]
extern crate vast_crypto;
#[cfg(any(test, feature = "test-helpers"))]
extern crate vastjson;
#[cfg(any(test, feature = "test-helpers"))]
extern crate kvdb_memorydb;
#[cfg(any(test, feature = "kvdb-rocksdb"))]
extern crate kvdb_rocksdb;
#[cfg(feature = "json-tests")]
#[macro_use]
extern crate lazy_static;
#[cfg(any(test, feature = "json-tests"))]
#[macro_use]
extern crate macros;
#[cfg(any(test, feature = "test-helpers"))]
extern crate pod;
#[cfg(any(test, feature = "blooms-db"))]
extern crate blooms_db;
#[cfg(feature = "env_logger")]
extern crate env_logger;
#[cfg(test)]
extern crate serde_json;
#[cfg(any(test, feature = "tempdir"))]
extern crate tempdir;

#[macro_use]
extern crate log;
#[macro_use]
extern crate trace_time;

#[cfg_attr(test, macro_use)]
extern crate evm;

#[cfg(all(test, feature = "price-info"))]
extern crate fetch;

#[cfg(all(test, feature = "price-info"))]
extern crate vast_runtime;

pub mod block;
pub mod client;
pub mod miner;

#[cfg(test)]
mod tests;
#[cfg(feature = "json-tests")]
pub mod json_tests;
#[cfg(any(test, feature = "test-helpers"))]
pub mod test_helpers;
