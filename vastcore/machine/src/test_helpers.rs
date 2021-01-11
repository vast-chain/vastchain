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

//! Provide facilities to create `Machine` instances for testing various networks.

use std::convert::TryFrom;
use common_types::engines::params::CommonParams;
use vastcore_builtin::Builtin;
use crate::Machine;

pub fn load_machine(reader: &[u8]) -> Machine {
	let spec = vastjson::spec::Spec::load(reader).expect("chain spec is invalid");

	let builtins = spec.accounts.builtins().into_iter().map(|p| (p.0.into(), Builtin::try_from(p.1).expect("chain spec is invalid"))).collect();
	let params = CommonParams::from(spec.params);

	if let vastjson::spec::Engine::Vastash(ref vastash) = spec.engine {
		Machine::with_vastash_extensions(params, builtins, vastash.params.clone().into())
	} else {
		Machine::regular(params, builtins)
	}
}

/// Create a new Foundation Frontier-era chain spec as though it never changes to Homestead.
pub fn new_frontier_test_machine() -> Machine { load_machine(include_bytes!("../../res/vast/frontier_test.json")) }

/// Create a new Foundation Homestead-era chain spec as though it never changed from Frontier.
pub fn new_homestead_test_machine() -> Machine { load_machine(include_bytes!("../../res/vast/homestead_test.json")) }

/// Create a new Foundation Homestead-EIP210-era chain spec as though it never changed from Homestead/Frontier.
pub fn new_eip210_test_machine() -> Machine { load_machine(include_bytes!("../../res/vast/eip210_test.json")) }

/// Create a new Foundation Byzantium era spec.
pub fn new_byzantium_test_machine() -> Machine { load_machine(include_bytes!("../../res/vast/byzantium_test.json")) }

/// Create a new Foundation Constantinople era spec.
pub fn new_constantinople_test_machine() -> Machine { load_machine(include_bytes!("../../res/vast/constantinople_test.json")) }

/// Create a new Foundation St. Peter's (Contantinople Fix) era spec.
pub fn new_constantinople_fix_test_machine() -> Machine { load_machine(include_bytes!("../../res/vast/st_peters_test.json")) }

/// Create a new Foundation Istanbul era spec.
pub fn new_istanbul_test_machine() -> Machine { load_machine(include_bytes!("../../res/vast/istanbul_test.json")) }

/// Create a new Musicoin-MCIP3-era spec.
pub fn new_mcip3_test_machine() -> Machine { load_machine(include_bytes!("../../res/vast/mcip3_test.json")) }

/// Create new Kovan spec with wasm activated at certain block
pub fn new_kovan_wasm_test_machine() -> Machine { load_machine(include_bytes!("../../res/vast/kovan_wasm_test.json")) }
