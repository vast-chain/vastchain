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

use std::ops::RangeInclusive;
use sync::ManageNetwork;
use self::vastcore_network::{ProtocolId, NetworkContext};

extern crate vastcore_network;

pub struct TestManageNetwork;

// TODO: rob, gavin (originally introduced this functions) - proper tests and test state
impl ManageNetwork for TestManageNetwork {
	fn accept_unreserved_peers(&self) { }
	fn deny_unreserved_peers(&self) { }
	fn remove_reserved_peer(&self, _peer: String) -> Result<(), String> { Ok(()) }
	fn add_reserved_peer(&self, _peer: String) -> Result<(), String> { Ok(()) }
	fn start_network(&self) {}
	fn stop_network(&self) {}
	fn num_peers_range(&self) -> RangeInclusive<u32> { 25..=50 }
	fn with_proto_context(&self, _: ProtocolId, _: &mut dyn FnMut(&dyn NetworkContext)) { }
}
