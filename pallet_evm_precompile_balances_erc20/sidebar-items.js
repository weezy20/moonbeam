initSidebarItems({"constant":[["SELECTOR_LOG_APPROVAL","Solidity selector of the Approval log, which is the Keccak of the Log signature."],["SELECTOR_LOG_DEPOSIT","Solidity selector of the Deposit log, which is the Keccak of the Log signature."],["SELECTOR_LOG_TRANSFER","Solidity selector of the Transfer log, which is the Keccak of the Log signature."],["SELECTOR_LOG_WITHDRAWAL","Solidity selector of the Withdraw log, which is the Keccak of the Log signature."]],"enum":[["Erc20BalancesPrecompileCall",""]],"struct":[["Erc20BalancesPrecompile","Precompile exposing a pallet_balance as an ERC20. Multiple precompiles can support instances of pallet_balance. The precompile uses an additional storage to store approvals."]],"trait":[["Erc20Metadata","Metadata of an ERC20 token."],["InstanceToPrefix","Associates pallet Instance to a prefix used for the Approves storage. This trait is implemented for () and the 16 substrate Instance."]],"type":[["ApprovesStorage","Storage type used to store approvals, since `pallet_balances` doesn’t handle this behavior. (Owner => Allowed => Amount)"],["BalanceOf","Alias for the Balance type for the provided Runtime and Instance."],["NoncesStorage","Storage type used to store EIP2612 nonces."]]});