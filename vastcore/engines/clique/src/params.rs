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

//! Clique specific parameters.

use vastjson;

/// `Clique` params.
pub struct CliqueParams {
	/// Period as defined in EIP
	pub period: u64,
	/// Epoch length as defined in EIP
	pub epoch: u64,
}

impl From<vastjson::spec::CliqueParams> for CliqueParams {
	fn from(p: vastjson::spec::CliqueParams) -> Self {
		let period = p.period.map_or_else(|| 30000 as u64, Into::into);
		let epoch =  p.epoch.map_or_else(|| 15 as u64, Into::into);

		assert!(epoch > 0);

		CliqueParams {
			period,
			epoch,
		}
	}
}
