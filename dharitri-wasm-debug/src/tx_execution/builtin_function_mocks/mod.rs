mod builtin_func_exec;
mod change_owner_mock;
mod dct_local_burn;
mod dct_local_mint;
mod dct_multi_transfer_mock;
mod dct_nft_add_quantity_mock;
mod dct_nft_burn_mock;
mod dct_nft_create_mock;
mod dct_nft_transfer_mock;
mod dct_transfer_mock;
mod set_username_mock;
mod upgrade_contract;

pub use builtin_func_exec::execute_builtin_function_or_default;
