use std::collections::BTreeMap;

use dharitri_wasm::types::Address;
use denali::model::{Account, AddressKey, BlockInfo, NewAddress};
use num_bigint::BigUint;

use crate::world_mock::{
    is_smart_contract_address, AccountData, AccountDct, BlockInfo as CrateBlockInfo,
    BlockchainMock, DctData, DctInstance, DctInstanceMetadata, DctInstances, DctRoles,
};

pub fn execute(
    state: &mut BlockchainMock,
    accounts: &BTreeMap<AddressKey, Account>,
    new_addresses: &[NewAddress],
    previous_block_info: &Option<BlockInfo>,
    current_block_info: &Option<BlockInfo>,
) {
    for (address, account) in accounts.iter() {
        let storage = account
            .storage
            .iter()
            .map(|(k, v)| (k.value.clone(), v.value.clone()))
            .collect();
        let dct = AccountDct::new_from_raw_map(
            account
                .dct
                .iter()
                .map(|(k, v)| {
                    (
                        k.value.clone(),
                        convert_denali_dct_to_world_mock(k.value.as_slice(), v),
                    )
                })
                .collect(),
        );

        state.validate_and_add_account(AccountData {
            address: address.value.into(),
            nonce: account
                .nonce
                .as_ref()
                .map(|nonce| nonce.value)
                .unwrap_or_default(),
            moax_balance: account
                .balance
                .as_ref()
                .map(|balance| balance.value.clone())
                .unwrap_or_default(),
            dct,
            username: account
                .username
                .as_ref()
                .map(|bytes_value| bytes_value.value.clone())
                .unwrap_or_default(),
            storage,
            contract_path: account
                .code
                .as_ref()
                .map(|bytes_value| bytes_value.value.clone()),
            contract_owner: account
                .owner
                .as_ref()
                .map(|address_value| address_value.value.into()),
        });
    }
    for new_address in new_addresses.iter() {
        assert!(
            is_smart_contract_address(&new_address.new_address.value.into()),
            "field should have SC format"
        );
        state.put_new_address(
            new_address.creator_address.value.into(),
            new_address.creator_nonce.value,
            new_address.new_address.value.into(),
        )
    }
    if let Some(block_info_obj) = &*previous_block_info {
        update_block_info(&mut state.previous_block_info, block_info_obj);
    }
    if let Some(block_info_obj) = &*current_block_info {
        update_block_info(&mut state.current_block_info, block_info_obj);
    }
}

fn convert_denali_dct_to_world_mock(
    token_identifier: &[u8],
    denali_dct: &denali::model::Dct,
) -> DctData {
    match denali_dct {
        denali::model::Dct::Short(short_dct) => {
            let balance = BigUint::from_bytes_be(short_dct.value.as_slice());
            let mut dct_data = DctData {
                token_identifier: token_identifier.to_vec(),
                ..Default::default()
            };
            dct_data.instances.add(0, balance);
            dct_data
        },
        denali::model::Dct::Full(full_dct) => DctData {
            token_identifier: full_dct
                .token_identifier
                .as_ref()
                .map(|token_identifier| token_identifier.value.clone())
                .unwrap_or_default(),
            instances: DctInstances::new_from_hash(
                full_dct
                    .instances
                    .iter()
                    .map(|denali_instance| {
                        let mock_instance =
                            convert_denali_dct_instance_to_world_mock(denali_instance);
                        (mock_instance.nonce, mock_instance)
                    })
                    .collect(),
            ),
            last_nonce: full_dct
                .last_nonce
                .as_ref()
                .map(|last_nonce| last_nonce.value)
                .unwrap_or_default(),
            roles: DctRoles::new(
                full_dct
                    .roles
                    .iter()
                    .map(|role| role.value.clone())
                    .collect(),
            ),
            frozen: if let Some(u64_value) = &full_dct.frozen {
                u64_value.value > 0
            } else {
                false
            },
        },
    }
}

fn convert_denali_dct_instance_to_world_mock(
    denali_dct: &denali::model::Instance,
) -> DctInstance {
    DctInstance {
        nonce: denali_dct
            .nonce
            .as_ref()
            .map(|nonce| nonce.value)
            .unwrap_or_default(),
        balance: denali_dct
            .balance
            .as_ref()
            .map(|value| value.value.clone())
            .unwrap_or_default(),
        metadata: DctInstanceMetadata {
            name: Vec::new(),
            creator: denali_dct
                .creator
                .as_ref()
                .map(|creator| Address::from_slice(creator.value.as_slice())),
            royalties: denali_dct
                .royalties
                .as_ref()
                .map(|royalties| royalties.value)
                .unwrap_or_default(),
            hash: denali_dct.hash.as_ref().map(|hash| hash.value.clone()),
            uri: denali_dct.uri.as_ref().map(|uri| uri.value.clone()),
            attributes: denali_dct
                .attributes
                .as_ref()
                .map(|attributes| attributes.value.clone())
                .unwrap_or_default(),
        },
    }
}

fn update_block_info(
    block_info: &mut CrateBlockInfo,
    denali_block_info: &denali::model::BlockInfo,
) {
    if let Some(u64_value) = &denali_block_info.block_timestamp {
        block_info.block_timestamp = u64_value.value;
    }
    if let Some(u64_value) = &denali_block_info.block_nonce {
        block_info.block_nonce = u64_value.value;
    }
    if let Some(u64_value) = &denali_block_info.block_epoch {
        block_info.block_epoch = u64_value.value;
    }
    if let Some(u64_value) = &denali_block_info.block_round {
        block_info.block_round = u64_value.value;
    }
    if let Some(bytes_value) = &denali_block_info.block_random_seed {
        const SEED_LEN: usize = 48;
        let val = &bytes_value.value;

        assert!(
            val.len() == SEED_LEN,
            "block random seed input value must be exactly 48 bytes long"
        );

        let mut seed = [0u8; SEED_LEN];
        seed[..].copy_from_slice(val.as_slice());
        block_info.block_random_seed = Box::from(seed);
    }
}
