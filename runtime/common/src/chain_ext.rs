use crate::*;
use dia_oracle::dia;
use scale_info::prelude::vec::Vec;
use sp_core::{Decode, Encode, MaxEncodedLen};
use sp_runtime::{codec, ArithmeticError, TokenError};

pub use spacewalk_primitives::{Asset, CurrencyId};

/// Address is a type alias for easier readability of address (accountId) communicated between contract and chain extension.
pub type Address = [u8; 32];
/// Amount is a type alias for easier readability of amount communicated between contract and chain extension.
pub type Amount = u128;
/// Blockchain is a type alias for easier readability of dia blockchain name communicated between contract and chain extension.
pub type Blockchain = [u8; 32];
/// Symbol is a type alias for easier readability of dia blockchain symbol communicated between contract and chain extension.
pub type Symbol = [u8; 32];

/// ChainExtensionError is almost the same as DispatchError, but with some modifications to make it compatible with being communicated between contract and chain extension. It implements the necessary From<T> conversions with DispatchError and other nested errors.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ChainExtensionError {
	/// Some error occurred.
	Other,
	/// Failed to lookup some data.
	CannotLookup,
	/// A bad origin.
	BadOrigin,
	/// A custom error in a module.
	Module,
	/// At least one consumer is remaining so the account cannot be destroyed.
	ConsumerRemaining,
	/// There are no providers so the account cannot be created.
	NoProviders,
	/// There are too many consumers so the account cannot be created.
	TooManyConsumers,
	/// An error to do with tokens.
	Token(ChainExtensionTokenError),
	/// An arithmetic error.
	Arithmetic(ChainExtensionArithmeticError),
	/// Unknown error
	Unknown,
}

/// ChainExtensionTokenError is a nested error in ChainExtensionError, similar to DispatchError's TokenError.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ChainExtensionTokenError {
	/// Funds are unavailable.
	NoFunds,
	/// Account that must exist would die.
	WouldDie,
	/// Account cannot exist with the funds that would be given.
	BelowMinimum,
	/// Account cannot be created.
	CannotCreate,
	/// The asset in question is unknown.
	UnknownAsset,
	/// Funds exist but are frozen.
	Frozen,
	/// Operation is not supported by the asset.
	Unsupported,
	/// Unknown error
	Unknown,
}

/// ChainExtensionArithmeticError is a nested error in ChainExtensionError, similar to DispatchError's ArithmeticError.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ChainExtensionArithmeticError {
	/// Underflow.
	Underflow,
	/// Overflow.
	Overflow,
	/// Division by zero.
	DivisionByZero,
	/// Unknown error
	Unknown,
}

impl From<DispatchError> for ChainExtensionError {
	fn from(e: DispatchError) -> Self {
		match e {
			DispatchError::Other(_) => ChainExtensionError::Other,
			DispatchError::CannotLookup => ChainExtensionError::CannotLookup,
			DispatchError::BadOrigin => ChainExtensionError::BadOrigin,
			DispatchError::Module(_) => ChainExtensionError::Module,
			DispatchError::ConsumerRemaining => ChainExtensionError::ConsumerRemaining,
			DispatchError::NoProviders => ChainExtensionError::NoProviders,
			DispatchError::TooManyConsumers => ChainExtensionError::TooManyConsumers,
			DispatchError::Token(token_err) =>
				ChainExtensionError::Token(ChainExtensionTokenError::from(token_err)),
			DispatchError::Arithmetic(arithmetic_error) => ChainExtensionError::Arithmetic(
				ChainExtensionArithmeticError::from(arithmetic_error),
			),
			_ => ChainExtensionError::Unknown,
		}
	}
}

impl From<TokenError> for ChainExtensionTokenError {
	fn from(e: TokenError) -> Self {
		match e {
			TokenError::NoFunds => ChainExtensionTokenError::NoFunds,
			TokenError::WouldDie => ChainExtensionTokenError::WouldDie,
			TokenError::BelowMinimum => ChainExtensionTokenError::BelowMinimum,
			TokenError::CannotCreate => ChainExtensionTokenError::CannotCreate,
			TokenError::UnknownAsset => ChainExtensionTokenError::UnknownAsset,
			TokenError::Frozen => ChainExtensionTokenError::Frozen,
			TokenError::Unsupported => ChainExtensionTokenError::Unsupported,
		}
	}
}

impl From<ArithmeticError> for ChainExtensionArithmeticError {
	fn from(e: ArithmeticError) -> Self {
		match e {
			ArithmeticError::Underflow => ChainExtensionArithmeticError::Underflow,
			ArithmeticError::Overflow => ChainExtensionArithmeticError::Overflow,
			ArithmeticError::DivisionByZero => ChainExtensionArithmeticError::DivisionByZero,
		}
	}
}

/// ToTrimmedVec is a trait implemented for [u8; 32] to allow both types Blockchain and Symbol (which are [u8; 32]) to have the trim_trailing_zeros function.
pub trait ToTrimmedVec {
	fn to_trimmed_vec(&self) -> Vec<u8>;
}
impl ToTrimmedVec for [u8; 32] {
	fn to_trimmed_vec(&self) -> Vec<u8> {
		trim_trailing_zeros(self).to_vec()
	}
}

/// trim_trailing_zeros takes an input slice and returns it without the trailing zeros.
fn trim_trailing_zeros(slice: &[u8]) -> &[u8] {
	let mut trim_amount = 0;
	for el in slice.iter().rev() {
		if *el == 0 {
			trim_amount += 1;
		} else {
			break
		}
	}
	&slice[..slice.len() - trim_amount]
}

/// CoinInfo is almost the same as Dia's CoinInfo, but with Encode, Decode, and TypeInfo which are necessary for contract to chain extension communication. Implements From<dia::CoinInfo> to make conversion.
#[derive(Debug, Clone, PartialEq, Eq, codec::Encode, codec::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct CoinInfo {
	pub symbol: Vec<u8>,
	pub name: Vec<u8>,
	pub blockchain: Vec<u8>,
	pub supply: u128,
	pub last_update_timestamp: u64,
	pub price: u128,
}
impl From<dia::CoinInfo> for CoinInfo {
	fn from(coin_info: dia::CoinInfo) -> Self {
		Self {
			symbol: coin_info.symbol,
			name: coin_info.name,
			blockchain: coin_info.blockchain,
			supply: coin_info.supply,
			last_update_timestamp: coin_info.last_update_timestamp,
			price: coin_info.price,
		}
	}
}


/// decode gets the slice from a Vec<u8> to decode it into its scale encoded type.
pub fn decode<T: Decode>(input: Vec<u8>) -> Result<T, codec::Error> {
	let mut input = input.as_slice();
	T::decode(&mut input)
}