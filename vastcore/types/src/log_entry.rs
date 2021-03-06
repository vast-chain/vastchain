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

//! Log entry type definition.

use std::ops::Deref;
use vast_util_mem::MallocSizeOf;
use bytes::Bytes;
use vast_types::{H256, Address, Bloom, BloomInput};

use {BlockNumber};
use vastjson;

/// A record of execution for a `LOG` operation.
#[derive(Default, Debug, Clone, PartialEq, Eq, RlpEncodable, RlpDecodable, MallocSizeOf)]
pub struct LogEntry {
	/// The address of the contract executing at the point of the `LOG` operation.
	pub address: Address,
	/// The topics associated with the `LOG` operation.
	pub topics: Vec<H256>,
	/// The data associated with the `LOG` operation.
	pub data: Bytes,
}

impl LogEntry {
	/// Calculates the bloom of this log entry.
	pub fn bloom(&self) -> Bloom {
		self.topics.iter().fold(Bloom::from(BloomInput::Raw(self.address.as_bytes())), |mut b, t| {
			b.accrue(BloomInput::Raw(t.as_bytes()));
			b
		})
	}
}

impl From<vastjson::state::Log> for LogEntry {
	fn from(l: vastjson::state::Log) -> Self {
		LogEntry {
			address: l.address.into(),
			topics: l.topics.into_iter().map(Into::into).collect(),
			data: l.data.into(),
		}
	}
}

/// Log localized in a blockchain.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct LocalizedLogEntry {
	/// Plain log entry.
	pub entry: LogEntry,
	/// Block in which this log was created.
	pub block_hash: H256,
	/// Block number.
	pub block_number: BlockNumber,
	/// Hash of transaction in which this log was created.
	pub transaction_hash: H256,
	/// Index of transaction within block.
	pub transaction_index: usize,
	/// Log position in the block.
	pub log_index: usize,
	/// Log position in the transaction.
	pub transaction_log_index: usize,
}

impl Deref for LocalizedLogEntry {
	type Target = LogEntry;

	fn deref(&self) -> &Self::Target {
		&self.entry
	}
}

#[cfg(test)]
mod tests {
	use vast_types::{Bloom, Address};
	use super::LogEntry;

	#[test]
	fn test_empty_log_bloom() {
		let bloom = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".parse::<Bloom>().unwrap();
		let address = "0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6".parse::<Address>().unwrap();
		let log = LogEntry {
			address: address,
			topics: vec![],
			data: vec![]
		};
		assert_eq!(log.bloom(), bloom);
	}
}
