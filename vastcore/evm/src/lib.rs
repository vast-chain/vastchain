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

//! Vast virtual machine.

extern crate bit_set;
extern crate vast_types;
extern crate parking_lot;
extern crate vast_util_mem;
extern crate vm;
extern crate keccak_hash as hash;
extern crate memory_cache;
extern crate vast_bytes as bytes;

#[macro_use]
extern crate lazy_static;

#[cfg_attr(feature = "evm-debug", macro_use)]
extern crate log;

#[cfg(test)]
extern crate rustc_hex;
#[cfg(test)]
extern crate hex_literal;

pub mod evm;
pub mod interpreter;

#[macro_use]
pub mod factory;
mod instructions;

#[cfg(test)]
mod tests;

pub use vm::{
    Schedule, CleanDustMode, EnvInfo, ActionType, ActionParams, Ext,
    ContractCreateResult, MessageCallResult, CreateContractAddress,
    GasLeft, ReturnData
};
pub use self::evm::{Finalize, FinalizationResult, CostType};
pub use self::instructions::{InstructionInfo, Instruction};
pub use self::factory::Factory;
