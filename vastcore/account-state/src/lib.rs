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

//! Account state
//! This crate contains code used to create, convert, and update Accounts and the code and storage
//! associated with it. It also defines the trait used to construct a backend to build a complete
//! caching state database.
//! Note: the code that needs access to `vastcore` types such as `Machine` and `Executive` is found in
//! the `executive_state` module in `vastcore`. Most tests for the `State` module in this crate are
//! also found in `executive_state` (for the same reason).

pub mod account;
pub mod backend;
pub mod state;

pub use {
	account::Account,
	backend::Backend,
	state::{State, CleanupMode},
};
