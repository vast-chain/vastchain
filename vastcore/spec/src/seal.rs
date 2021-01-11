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

//! Spec seal.

use rlp::RlpStream;
use vast_types::{H64, H256, H520};
use vastjson;

/// Classic vast seal.
#[derive(Debug)]
pub struct Vast {
	/// Seal nonce.
	pub nonce: H64,
	/// Seal mix hash.
	pub mix_hash: H256,
}

impl Into<Generic> for Vast {
	fn into(self) -> Generic {
		let mut s = RlpStream::new_list(2);
		s.append(&self.mix_hash).append(&self.nonce);
		Generic(s.out())
	}
}

/// AuthorityRound seal.
#[derive(Debug)]
pub struct AuthorityRound {
	/// Seal step.
	pub step: usize,
	/// Seal signature.
	pub signature: H520,
}

/// Tendermint seal.
#[derive(Debug)]
pub struct Tendermint {
	/// Seal round.
	pub round: usize,
	/// Proposal seal signature.
	pub proposal: H520,
	/// Precommit seal signatures.
	pub precommits: Vec<H520>,
}

impl Into<Generic> for AuthorityRound {
	fn into(self) -> Generic {
		let mut s = RlpStream::new_list(2);
		s.append(&self.step).append(&self.signature);
		Generic(s.out())
	}
}

impl Into<Generic> for Tendermint {
	fn into(self) -> Generic {
		let mut stream = RlpStream::new_list(3);
		stream
			.append(&self.round)
			.append(&self.proposal)
			.append_list(&self.precommits);
		Generic(stream.out())
	}
}

#[derive(Debug)]
pub struct Generic(pub Vec<u8>);

/// Genesis seal type.
#[derive(Debug)]
pub enum Seal {
	/// Classic vast seal.
	Vast(Vast),
	/// AuthorityRound seal.
	AuthorityRound(AuthorityRound),
	/// Tendermint seal.
	Tendermint(Tendermint),
	/// Generic RLP seal.
	Generic(Generic),
}

impl From<vastjson::spec::Seal> for Seal {
	fn from(s: vastjson::spec::Seal) -> Self {
		match s {
			vastjson::spec::Seal::Vast(vast) => Seal::Vast(Vast {
				nonce: vast.nonce.into(),
				mix_hash: vast.mix_hash.into()
			}),
			vastjson::spec::Seal::AuthorityRound(ar) => Seal::AuthorityRound(AuthorityRound {
				step: ar.step.into(),
				signature: ar.signature.into()
			}),
			vastjson::spec::Seal::Tendermint(tender) => Seal::Tendermint(Tendermint {
				round: tender.round.into(),
				proposal: tender.proposal.into(),
				precommits: tender.precommits.into_iter().map(Into::into).collect()
			}),
			vastjson::spec::Seal::Generic(g) => Seal::Generic(Generic(g.into())),
		}
	}
}

impl Into<Generic> for Seal {
	fn into(self) -> Generic {
		match self {
			Seal::Generic(generic) => generic,
			Seal::Vast(vast) => vast.into(),
			Seal::AuthorityRound(ar) => ar.into(),
			Seal::Tendermint(tender) => tender.into(),
		}
	}
}