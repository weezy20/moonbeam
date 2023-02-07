// Copyright 2019-2022 PureStake Inc.
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
use crate::{mock::*, SELECTOR_LOG_PREIMAGE_NOTED, SELECTOR_LOG_PREIMAGE_UNNOTED};
use precompile_utils::{prelude::*, testing::*, EvmDataWriter};

use frame_support::{assert_ok, dispatch::Dispatchable};
use pallet_evm::{Call as EvmCall, Event as EvmEvent};

use sp_core::{Hasher, H256, U256};

fn evm_call(input: Vec<u8>) -> EvmCall<Runtime> {
	EvmCall::call {
		source: Alice.into(),
		target: Precompile1.into(),
		input,
		value: U256::zero(),
		gas_limit: u64::max_value(),
		max_fee_per_gas: 0.into(),
		max_priority_fee_per_gas: Some(U256::zero()),
		nonce: None,
		access_list: Vec::new(),
	}
}

#[test]
fn test_solidity_interface_has_all_function_selectors_documented_and_implemented() {
	for file in ["Preimage.sol"] {
		for solidity_fn in solidity::get_selectors(file) {
			assert_eq!(
				solidity_fn.compute_selector_hex(),
				solidity_fn.docs_selector,
				"documented selector for '{}' did not match for file '{}'",
				solidity_fn.signature(),
				file,
			);

			let selector = solidity_fn.compute_selector();
			if !PCall::supports_selector(selector) {
				panic!(
					"failed decoding selector 0x{:x} => '{}' as Action for file '{}'",
					selector,
					solidity_fn.signature(),
					file,
				)
			}
		}
	}
}

#[test]
fn note_unnote_preimage_logs_work() {
	ExtBuilder::default()
		.with_balances(vec![(Alice.into(), 100_000)])
		.build()
		.execute_with(|| {
			let bytes = vec![1, 2, 3];
			let expected_hash = sp_runtime::traits::BlakeTwo256::hash(&bytes);

			// Note preimage
			let input = PCall::note_preimage {
				encoded_proposal: bytes.into(),
			}
			.into();
			assert_ok!(RuntimeCall::Evm(evm_call(input)).dispatch(RuntimeOrigin::root()));

			// Assert note preimage event is emited and matching frame event preimage hash.
			assert!(vec![
				EvmEvent::Log {
					log: log1(
						Precompile1,
						SELECTOR_LOG_PREIMAGE_NOTED,
						EvmDataWriter::new().write::<H256>(expected_hash).build(),
					),
				}
				.into(),
				RuntimeEvent::Preimage(pallet_preimage::pallet::Event::Noted {
					hash: expected_hash
				})
			]
			.iter()
			.all(|log| events().contains(log)));

			// Unnote preimage
			let input = PCall::unnote_preimage {
				hash: expected_hash,
			}
			.into();
			assert_ok!(RuntimeCall::Evm(evm_call(input)).dispatch(RuntimeOrigin::root()));

			// Assert unnote preimage is emited
			assert!(events().contains(
				&EvmEvent::Log {
					log: log1(
						Precompile1,
						SELECTOR_LOG_PREIMAGE_UNNOTED,
						EvmDataWriter::new().write::<H256>(expected_hash).build(),
					),
				}
				.into()
			));
		})
}
