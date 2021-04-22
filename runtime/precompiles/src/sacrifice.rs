// Copyright 2019-2021 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Decode;
use evm::{Context, ExitError, ExitSucceed};
use frame_support::dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo};
use pallet_evm::{Config, Precompile, PrecompileSet};
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_dispatch::Dispatch;
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_simple::{ECRecover, Identity, Ripemd160, Sha256};
use sp_core::H160;
use sp_std::{marker::PhantomData, vec::Vec};

/// A precompile intended to burn gas and/or time without actually doing any work.
/// Meant for testing.
///
/// Expects call data to include two u64 values:
/// 1) The gas to sacrifice (charge)
/// 2) The time in msec to sleep
/// TODO: use feature flags / somehow prevent this from deployment onto live networks (or not?)
pub struct Sacrifice;

impl Precompile for Sacrifice {
	fn execute(
		input: &[u8],
		target_gas: Option<u64>,
		_context: &Context,
	) -> core::result::Result<(ExitSucceed, Vec<u8>, u64), ExitError> {
		const INPUT_SIZE_BYTES: usize = 8;

		// input should be exactly 8 bytes (one 64-bit unsigned int)
		if input.len() != INPUT_SIZE_BYTES {
			log::warn!("Input: {:?}", input);
			return Err(ExitError::Other(
				"input length for Sacrifice must be exactly 8 bytes".into()));
		}

		// create 8-byte buffers and populate them from calldata...
		let mut gas_cost_buf: [u8; 8] = [0; 8];
		gas_cost_buf.copy_from_slice(&input[0..8]);

		// then read them into a u64 as big-endian...
		let gas_cost = u64::from_be_bytes(gas_cost_buf);

		// ensure we can afford our sacrifice...
		if let Some(gas_left) = target_gas {
			if gas_left < gas_cost {
				return Err(ExitError::OutOfGas);
			}
		}
		
		Ok((ExitSucceed::Returned, [0u8; 0].to_vec(), gas_cost))
	}
}

/// The PrecompileSet installed in the Moonbeam runtime.
/// We include the nine Istanbul precompiles
/// (https://github.com/ethereum/go-ethereum/blob/3c46f557/core/vm/contracts.go#L69)
/// as well as a special precompile for dispatching Substrate extrinsics
#[derive(Debug, Clone, Copy)]
pub struct MoonbeamPrecompiles<R>(PhantomData<R>);

impl<R> PrecompileSet for MoonbeamPrecompiles<R>
where
	R: Config,
	R::Call: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo + Decode,
	<R::Call as Dispatchable>::Origin: From<Option<R::AccountId>>,
{
	fn execute(
		address: H160,
		input: &[u8],
		target_gas: Option<u64>,
		context: &Context,
	) -> Option<Result<(ExitSucceed, Vec<u8>, u64), ExitError>> {
		match address {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(input, target_gas, context)),
			a if a == hash(2) => Some(Sha256::execute(input, target_gas, context)),
			a if a == hash(3) => Some(Ripemd160::execute(input, target_gas, context)),
			a if a == hash(4) => Some(Identity::execute(input, target_gas, context)),
			a if a == hash(5) => Some(Modexp::execute(input, target_gas, context)),
			a if a == hash(6) => Some(Bn128Add::execute(input, target_gas, context)),
			a if a == hash(7) => Some(Bn128Mul::execute(input, target_gas, context)),
			a if a == hash(8) => Some(Bn128Pairing::execute(input, target_gas, context)),
			// Moonbeam precompiles :
			a if a == hash(255) => Some(Dispatch::<R>::execute(input, target_gas, context)),
			// Moonbeam testing-only precompile(s):
			a if a == hash(511) => Some(Sacrifice::execute(input, target_gas, context)),
			_ => None,
		}
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::time::{Duration, Instant};
	extern crate hex;

	#[test]
	fn test_invalid_input_length() -> std::result::Result<(), ExitError> {
		let cost: u64 = 1;

		// TODO: this is very not-DRY, it would be nice to have a default / test impl in Frontier
		let context: Context = Context {
			address: Default::default(),
			caller: Default::default(),
			apparent_value: From::from(0),
		};

		// should fail with input of 7 byte length
		let input: [u8; 7] = [0; 7];
		assert_eq!(
			Sacrifice::execute(&input, Some(cost), &context),
			Err(ExitError::Other("input length for Sacrifice must be exactly 8 bytes".into())),
		);

		// should fail with input of 9 byte length
		let input: [u8; 9] = [0; 9];
		assert_eq!(
			Sacrifice::execute(&input, Some(cost), &context),
			Err(ExitError::Other("input length for Sacrifice must be exactly 8 bytes".into())),
		);

		Ok(())
	}

	#[test]
	fn test_gas_consumption() -> std::result::Result<(), ExitError> {
		let mut input: [u8; 8] = [0; 8];
		input[..8].copy_from_slice(&123456_u64.to_be_bytes());

		let context: Context = Context {
			address: Default::default(),
			caller: Default::default(),
			apparent_value: From::from(0),
		};

		assert_eq!(
			Sacrifice::execute(&input, None, &context),
			Ok((ExitSucceed::Returned, [0u8; 0].to_vec(), 123456)),
		);

		Ok(())
	}

	#[test]
	fn test_oog() -> std::result::Result<(), ExitError> {
		let mut input: [u8; 8] = [0; 8];
		input[..8].copy_from_slice(&100_u64.to_be_bytes());

		let context: Context = Context {
			address: Default::default(),
			caller: Default::default(),
			apparent_value: From::from(0),
		};

		assert_eq!(
			Sacrifice::execute(&input, Some(99), &context),
			Err(ExitError::OutOfGas),
		);

		Ok(())
	}
}
