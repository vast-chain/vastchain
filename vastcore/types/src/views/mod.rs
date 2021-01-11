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

//! Block oriented views onto rlp.

#[macro_use]
mod view_rlp;
mod block;
mod body;
mod header;
mod transaction;

pub use self::view_rlp::ViewRlp;
pub use self::block::BlockView;
pub use self::body::BodyView;
pub use self::header::HeaderView;
pub use self::transaction::TransactionView;

#[cfg(test)]
mod tests {
	use super::HeaderView;

	#[test]
	#[should_panic]
	fn should_include_file_line_number_in_panic_for_invalid_rlp() {
		let _ = view!(HeaderView, &[]).parent_hash();
	}
}
