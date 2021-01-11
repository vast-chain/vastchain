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

// #![warn(missing_docs)]

extern crate edit_distance;
extern crate vast_crypto;
extern crate vast_wordlist;
extern crate serde;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod brain;
mod brain_prefix;
mod password;
mod prefix;

pub mod brain_recover;

pub use self::vast_wordlist::Error as WordlistError;
pub use self::brain::Brain;
pub use self::brain_prefix::BrainPrefix;
pub use self::password::Password;
pub use self::prefix::Prefix;
