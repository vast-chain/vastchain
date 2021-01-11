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

//! RPC implementations for the light client.
//!
//! This doesn't re-implement all of the RPC APIs, just those which aren't
//! significantly generic to be reused.

pub mod vast;
pub mod vast;
pub mod vast_set;
pub mod trace;
pub mod net;

pub use self::vast::VastClient;
pub use self::vast::VastClient;
pub use self::vast_set::VastSetClient;
pub use self::net::NetClient;
pub use self::trace::TracesClient;
