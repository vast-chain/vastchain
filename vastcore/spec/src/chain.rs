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

//! Load chain specifications for all chains supported by the vast-vast client.

macro_rules! bundle_release_spec {
	($($path: expr => $name: ident), *) => {
		$(
			/// Bundled release spec
			pub fn $name<'a, T: Into<crate::spec::SpecParams<'a>>>(params: T) -> crate::spec::Spec {
				let params = params.into();
				crate::spec::Spec::load(
					params,
					include_bytes!(concat!("../../res/", $path, ".json")) as &[u8]
				).expect(concat!("Chain spec ", $path, " is invalid."))
			}
		)*
	}
}

macro_rules! bundle_test_spec {
	($($path: expr => $name: ident), *) => {
		$(
			/// Bundled test spec
			pub fn $name() -> crate::spec::Spec {
				crate::spec::Spec::load(
					&::std::env::temp_dir(),
					include_bytes!(concat!("../../res/", $path, ".json")) as &[u8]
				).expect(concat!("Chain spec ", $path, " is invalid."))
			}
		)*
	}
}

macro_rules! bundle_test_machine {
	($($path: expr => $name: ident), *) => {
		$(
			/// Bundled test spec
			pub fn $name() -> machine::Machine {
				crate::spec::Spec::load_machine(
					include_bytes!(concat!("../../res/", $path, ".json")) as &[u8]
				).expect(concat!("Chain spec ", $path, " is invalid."))
			}
		)*
	}
}

bundle_release_spec! {
	"vast/callisto" => new_callisto,
	"vast/classic" => new_classic,
	"vast/ellaism" => new_ellaism,
	"vast/evantestcore" => new_evantestcore,
	"vast/evancore" => new_evancore,
	"vast/expanse" => new_expanse,
	"vast/foundation" => new_foundation,
	"vast/goerli" => new_goerli,
	"vast/kotti" => new_kotti,
	"vast/kovan" => new_kovan,
	"vast/mix" => new_mix,
	"vast/morden" => new_morden,
	"vast/mordor" => new_mordor,
	"vast/musicoin" => new_musicoin,
	"vast/poacore" => new_poanet,
	"vast/xdai" => new_xdai,
	"vast/vastercore" => new_vastercore,
	"vast/poasokol" => new_sokol,
	"vast/rinkeby" => new_rinkeby,
	"vast/ropsten" => new_ropsten,
	"vast/volta" => new_volta,
	"vast/ewc" => new_ewc
}

bundle_test_spec! {
	"authority_round" => new_test_round,
	"authority_round_block_reward_contract" => new_test_round_block_reward_contract,
	"authority_round_empty_steps" => new_test_round_empty_steps,
	"authority_round_randomness_contract" => new_test_round_randomness_contract,
	"constructor" => new_test_constructor,
	"vast/byzantium_test" => new_byzantium_test,
	"vast/constantinople_test" => new_constantinople_test,
	"vast/istanbul_test" => new_istanbul_test,
	"vast/eip150_test" => new_eip150_test,
	"vast/eip161_test" => new_eip161_test,
	"vast/eip210_test" => new_eip210_test,
	"vast/frontier_like_test" => new_mainnet_like,
	"vast/frontier_test" => new_frontier_test,
	"vast/homestead_test" => new_homestead_test,
	"vast/kovan_wasm_test" => new_kovan_wasm_test,
	"vast/mcip3_test" => new_mcip3_test,
	"vast/morden" => new_morden_test,
	"vast/mordor" => new_mordor_test,
	"vast/ropsten" => new_ropsten_test,
	"vast/st_peters_test" => new_constantinople_fix_test,
	"vast/transition_test" => new_transition_test,
	"instant_seal" => new_instant,
	"null" => new_null,
	"null_morden" => new_test,
	"null_morden_with_reward" => new_test_with_reward,
	"null_morden_with_finality" => new_test_with_finality,
	"validator_contract" => new_validator_contract,
	"validator_multi" => new_validator_multi,
	"validator_safe_contract" => new_validator_safe_contract
}

bundle_test_machine! {
	"vast/byzantium_test" => new_byzantium_test_machine,
	"vast/constantinople_test" => new_constantinople_test_machine,
	"vast/istanbul_test" => new_istanbul_test_machine,
	"vast/eip210_test" => new_eip210_test_machine,
	"vast/frontier_test" => new_frontier_test_machine,
	"vast/homestead_test" => new_homestead_test_machine,
	"vast/kovan_wasm_test" => new_kovan_wasm_test_machine,
	"null_morden" => new_test_machine
}

#[cfg(test)]
mod tests {
	use account_state::State;
	use common_types::{view, views::BlockView};
	use vast_types::U256;
	use tempdir::TempDir;
	use vastcore::test_helpers::get_temp_state_db;

	use super::{new_morden, new_foundation};

	#[test]
	fn ensure_db_good() {
		let tempdir = TempDir::new("").unwrap();
		let spec = new_morden(&tempdir.path());
		let engine = &spec.engine;
		let genesis_header = spec.genesis_header();
		let db = spec.ensure_db_good(get_temp_state_db(), &Default::default()).unwrap();
		let s = State::from_existing(db, genesis_header.state_root().clone(), engine.account_start_nonce(0), Default::default()).unwrap();
		assert_eq!(s.balance(&"0000000000000000000000000000000000000001".parse().unwrap()).unwrap(), 1u64.into());
		assert_eq!(s.balance(&"0000000000000000000000000000000000000002".parse().unwrap()).unwrap(), 1u64.into());
		assert_eq!(s.balance(&"0000000000000000000000000000000000000003".parse().unwrap()).unwrap(), 1u64.into());
		assert_eq!(s.balance(&"0000000000000000000000000000000000000004".parse().unwrap()).unwrap(), 1u64.into());
		assert_eq!(s.balance(&"102e61f5d8f9bc71d0ad4a084df4e65e05ce0e1c".parse().unwrap()).unwrap(), U256::from(1u64) << 200);
		assert_eq!(s.balance(&"0000000000000000000000000000000000000000".parse().unwrap()).unwrap(), 0u64.into());
	}

	#[test]
	fn morden() {
		let tempdir = TempDir::new("").unwrap();
		let morden = new_morden(&tempdir.path());

		assert_eq!(morden.state_root, "f3f4696bbf3b3b07775128eb7a3763279a394e382130f27c21e70233e04946a9".parse().unwrap());
		let genesis = morden.genesis_block();
		assert_eq!(view!(BlockView, &genesis).header_view().hash(), "0cd786a2425d16f152c658316c423e6ce1181e15c3295826d7c9904cba9ce303".parse().unwrap());
	}

	#[test]
	fn frontier() {
		let tempdir = TempDir::new("").unwrap();
		let frontier = new_foundation(&tempdir.path());

		assert_eq!(frontier.state_root, "d7f8974fb5ac78d9ac099b9ad5018bedc2ce0a72dad1827a1709da30580f0544".parse().unwrap());
		let genesis = frontier.genesis_block();
		assert_eq!(view!(BlockView, &genesis).header_view().hash(), "d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3".parse().unwrap());
	}
}
