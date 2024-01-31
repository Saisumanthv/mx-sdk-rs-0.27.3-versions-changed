use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

use super::{
    change_owner_mock::execute_change_owner, dct_local_burn::execute_local_burn,
    dct_local_mint::execute_local_mint, dct_multi_transfer_mock::execute_dct_multi_transfer,
    dct_nft_add_quantity_mock::execute_nft_add_quantity, dct_nft_burn_mock::execute_nft_burn,
    dct_nft_create_mock::execute_dct_nft_create,
    dct_nft_transfer_mock::execute_dct_nft_transfer, dct_transfer_mock::execute_dct_transfer,
    set_username_mock::execute_set_username, upgrade_contract::execute_upgrade_contract,
};

use dharitri_wasm::api::{
    CHANGE_OWNER_BUILTIN_FUNC_NAME, DCT_LOCAL_BURN_FUNC_NAME, DCT_LOCAL_MINT_FUNC_NAME,
    DCT_MULTI_TRANSFER_FUNC_NAME, DCT_NFT_ADD_QUANTITY_FUNC_NAME, DCT_NFT_BURN_FUNC_NAME,
    DCT_NFT_CREATE_FUNC_NAME, DCT_NFT_TRANSFER_FUNC_NAME, DCT_TRANSFER_FUNC_NAME,
    SET_USERNAME_FUNC_NAME, UPGRADE_CONTRACT_FUNC_NAME,
};

const DCT_ROLE_LOCAL_MINT: &[u8] = b"DCTRoleLocalMint";
const DCT_ROLE_LOCAL_BURN: &[u8] = b"DCTRoleLocalBurn";
const DCT_ROLE_NFT_CREATE: &[u8] = b"DCTRoleNFTCreate";
const DCT_ROLE_NFT_ADD_QUANTITY: &[u8] = b"DCTRoleNFTAddQuantity";
const DCT_ROLE_NFT_BURN: &[u8] = b"DCTRoleNFTBurn";

pub fn execute_builtin_function_or_default(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    match tx_input.func_name.as_slice() {
        DCT_LOCAL_MINT_FUNC_NAME => check_and_execute_builtin_function(
            DCT_ROLE_LOCAL_MINT,
            tx_input,
            tx_cache,
            &execute_local_mint,
        ),
        DCT_LOCAL_BURN_FUNC_NAME => check_and_execute_builtin_function(
            DCT_ROLE_LOCAL_BURN,
            tx_input,
            tx_cache,
            &execute_local_burn,
        ),
        DCT_MULTI_TRANSFER_FUNC_NAME => execute_dct_multi_transfer(tx_input, tx_cache),
        DCT_NFT_TRANSFER_FUNC_NAME => execute_dct_nft_transfer(tx_input, tx_cache),
        DCT_NFT_CREATE_FUNC_NAME => check_and_execute_builtin_function(
            DCT_ROLE_NFT_CREATE,
            tx_input,
            tx_cache,
            &execute_dct_nft_create,
        ),
        DCT_NFT_ADD_QUANTITY_FUNC_NAME => check_and_execute_builtin_function(
            DCT_ROLE_NFT_ADD_QUANTITY,
            tx_input,
            tx_cache,
            &execute_nft_add_quantity,
        ),
        DCT_NFT_BURN_FUNC_NAME => check_and_execute_builtin_function(
            DCT_ROLE_NFT_BURN,
            tx_input,
            tx_cache,
            &execute_nft_burn,
        ),

        DCT_TRANSFER_FUNC_NAME => execute_dct_transfer(tx_input, tx_cache),
        CHANGE_OWNER_BUILTIN_FUNC_NAME => execute_change_owner(tx_input, tx_cache),
        SET_USERNAME_FUNC_NAME => execute_set_username(tx_input, tx_cache),
        UPGRADE_CONTRACT_FUNC_NAME => execute_upgrade_contract(tx_input, tx_cache),
        _ => default_execution(tx_input, tx_cache),
    }
}

fn check_and_execute_builtin_function(
    role_name: &[u8],
    tx_input: TxInput,
    tx_cache: TxCache,
    f: &dyn Fn(TxInput, TxCache) -> (TxResult, BlockchainUpdate),
) -> (TxResult, BlockchainUpdate) {
    let check_result = check_allowed_to_execute(role_name, &tx_input, &tx_cache);
    if let Some(tx_result) = check_result {
        return (tx_result, BlockchainUpdate::empty());
    }
    f(tx_input, tx_cache)
}

pub fn check_allowed_to_execute(
    builtin_function_name: &[u8],
    tx_input: &TxInput,
    tx_cache: &TxCache,
) -> Option<TxResult> {
    let token_identifier = tx_input.args[0].clone();
    let available_roles = tx_cache.with_account_mut(&tx_input.to, |account| {
        account.dct.get_roles(&token_identifier)
    });
    if available_roles.contains(&builtin_function_name.to_vec()) {
        return None;
    }

    Some(TxResult::from_vm_error("action is not allowed".to_string()))
}
