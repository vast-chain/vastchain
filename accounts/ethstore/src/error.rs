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

use std::fmt;
use std::io::Error as IoError;
use crypto::{self, Error as VastCryptoError};
use crypto::publickey::{Error as VastPublicKeyCryptoError, DerivationError};

/// Account-related errors.
#[derive(Debug)]
pub enum Error {
	/// IO error
	Io(IoError),
	/// Invalid Password
	InvalidPassword,
	/// Account's secret is invalid.
	InvalidSecret,
	/// Invalid Vault Crypto meta.
	InvalidCryptoMeta,
	/// Invalid Account.
	InvalidAccount,
	/// Invalid Message.
	InvalidMessage,
	/// Invalid Key File
	InvalidKeyFile(String),
	/// Vaults are not supported.
	VaultsAreNotSupported,
	/// Unsupported vault
	UnsupportedVault,
	/// Invalid vault name
	InvalidVaultName,
	/// Vault not found
	VaultNotFound,
	/// Account creation failed.
	CreationFailed,
	/// `VastCrypto` error
	VastCrypto(VastCryptoError),
	/// `VastPublicKeyCryptoError` error
	VastPublicKeyCrypto(VastPublicKeyCryptoError),
	/// Derivation error
	Derivation(DerivationError),
	/// Custom error
	Custom(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		let s = match *self {
			Error::Io(ref err) => err.to_string(),
			Error::InvalidPassword => "Invalid password".into(),
			Error::InvalidSecret => "Invalid secret".into(),
			Error::InvalidCryptoMeta => "Invalid crypted metadata".into(),
			Error::InvalidAccount => "Invalid account".into(),
			Error::InvalidMessage => "Invalid message".into(),
			Error::InvalidKeyFile(ref reason) => format!("Invalid key file: {}", reason),
			Error::VaultsAreNotSupported => "Vaults are not supported".into(),
			Error::UnsupportedVault => "Vault is not supported for this operation".into(),
			Error::InvalidVaultName => "Invalid vault name".into(),
			Error::VaultNotFound => "Vault not found".into(),
			Error::CreationFailed => "Account creation failed".into(),
			Error::VastCrypto(ref err) => err.to_string(),
			Error::VastPublicKeyCrypto(ref err) => err.to_string(),
			Error::Derivation(ref err) => format!("Derivation error: {:?}", err),
			Error::Custom(ref s) => s.clone(),
		};

		write!(f, "{}", s)
	}
}

impl From<IoError> for Error {
	fn from(err: IoError) -> Self {
		Error::Io(err)
	}
}

impl From<VastPublicKeyCryptoError> for Error {
	fn from(err: VastPublicKeyCryptoError) -> Self {
		Error::VastPublicKeyCrypto(err)
	}
}

impl From<VastCryptoError> for Error {
	fn from(err: VastCryptoError) -> Self {
		Error::VastCrypto(err)
	}
}

impl From<crypto::error::ScryptError> for Error {
	fn from(err: crypto::error::ScryptError) -> Self {
		Error::VastCrypto(err.into())
	}
}

impl From<crypto::error::SymmError> for Error {
	fn from(err: crypto::error::SymmError) -> Self {
		Error::VastCrypto(err.into())
	}
}

impl From<DerivationError> for Error {
	fn from(err: DerivationError) -> Self {
		Error::Derivation(err)
	}
}
