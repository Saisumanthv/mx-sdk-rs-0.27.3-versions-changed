#[test]
fn big_int_to_i64_go() {
    dharitri_wasm_debug::denali_go("denali/big_int_to_i64.scen.json");
}

#[test]
fn big_num_conversions_go() {
    dharitri_wasm_debug::denali_go("denali/big_num_conversions.scen.json");
}

#[test]
fn big_uint_sqrt_go() {
    dharitri_wasm_debug::denali_go("denali/big_uint_sqrt.scen.json");
}

#[test]
fn big_uint_to_u64_go() {
    dharitri_wasm_debug::denali_go("denali/big_uint_to_u64.scen.json");
}

#[test]
fn block_info_go() {
    dharitri_wasm_debug::denali_go("denali/block_info.scen.json");
}

#[test]
fn boxed_bytes_zeros_go() {
    dharitri_wasm_debug::denali_go("denali/boxed_bytes_zeros.scen.json");
}

#[test]
fn codec_err_go() {
    dharitri_wasm_debug::denali_go("denali/codec_err.scen.json");
}

#[test]
fn count_ones_go() {
    dharitri_wasm_debug::denali_go("denali/count_ones.scen.json");
}

#[test]
fn crypto_elliptic_curves_go() {
    dharitri_wasm_debug::denali_go("denali/crypto_elliptic_curves.scen.json");
}

#[test]
fn crypto_keccak256_legacy_go() {
    dharitri_wasm_debug::denali_go("denali/crypto_keccak256_legacy.scen.json");
}

// TODO: enable after new VM release is out.
// #[test]
// fn crypto_keccak256_go() {
//     dharitri_wasm_debug::denali_go("denali/crypto_keccak256.scen.json");
// }

#[test]
fn crypto_ripemd160_go() {
    dharitri_wasm_debug::denali_go("denali/crypto_ripemd160.scen.json");
}

#[test]
fn crypto_sha256_legacy_go() {
    dharitri_wasm_debug::denali_go("denali/crypto_sha256_legacy.scen.json");
}

// TODO: enable after new VM release is out.
// #[test]
// fn crypto_sha256_go() {
//     dharitri_wasm_debug::denali_go("denali/crypto_sha256.scen.json");
// }

#[test]
fn crypto_verify_funcs_go() {
    dharitri_wasm_debug::denali_go("denali/crypto_verify_funcs.scen.json");
}

#[test]
fn echo_array_u8_go() {
    dharitri_wasm_debug::denali_go("denali/echo_array_u8.scen.json");
}

#[test]
fn echo_arrayvec_go() {
    dharitri_wasm_debug::denali_go("denali/echo_arrayvec.scen.json");
}

#[test]
fn echo_async_result_empty_go() {
    dharitri_wasm_debug::denali_go("denali/echo_async_result_empty.scen.json");
}

#[test]
fn echo_async_result_empty_managed_go() {
    dharitri_wasm_debug::denali_go("denali/echo_async_result_empty_managed.scen.json");
}

#[test]
fn echo_big_int_nested_go() {
    dharitri_wasm_debug::denali_go("denali/echo_big_int_nested.scen.json");
}

#[test]
fn echo_big_int_top_go() {
    dharitri_wasm_debug::denali_go("denali/echo_big_int_top.scen.json");
}

#[test]
fn echo_big_uint_go() {
    dharitri_wasm_debug::denali_go("denali/echo_big_uint.scen.json");
}

#[test]
fn echo_boxed_bytes_go() {
    dharitri_wasm_debug::denali_go("denali/echo_boxed_bytes.scen.json");
}

#[test]
fn echo_i32_go() {
    dharitri_wasm_debug::denali_go("denali/echo_i32.scen.json");
}

#[test]
fn echo_i64_go() {
    dharitri_wasm_debug::denali_go("denali/echo_i64.scen.json");
}

#[test]
fn echo_ignore_go() {
    dharitri_wasm_debug::denali_go("denali/echo_ignore.scen.json");
}

#[test]
fn echo_managed_bytes_go() {
    dharitri_wasm_debug::denali_go("denali/echo_managed_bytes.scen.json");
}

#[test]
fn echo_managed_vec_go() {
    dharitri_wasm_debug::denali_go("denali/echo_managed_vec.scen.json");
}

#[test]
fn echo_nothing_go() {
    dharitri_wasm_debug::denali_go("denali/echo_nothing.scen.json");
}

#[test]
fn echo_ser_ex_1_go() {
    dharitri_wasm_debug::denali_go("denali/echo_ser_ex_1.scen.json");
}

#[test]
fn echo_slice_u8_go() {
    dharitri_wasm_debug::denali_go("denali/echo_slice_u8.scen.json");
}

#[test]
fn echo_str_go() {
    dharitri_wasm_debug::denali_go("denali/echo_str.scen.json");
}

#[test]
fn echo_str_box_go() {
    dharitri_wasm_debug::denali_go("denali/echo_str_box.scen.json");
}

#[test]
fn echo_string_go() {
    dharitri_wasm_debug::denali_go("denali/echo_string.scen.json");
}

#[test]
fn echo_tuple_into_multiresult_go() {
    dharitri_wasm_debug::denali_go("denali/echo_tuple_into_multiresult.scen.json");
}

#[test]
fn echo_u64_go() {
    dharitri_wasm_debug::denali_go("denali/echo_u64.scen.json");
}

#[test]
fn echo_usize_go() {
    dharitri_wasm_debug::denali_go("denali/echo_usize.scen.json");
}

#[test]
fn echo_varargs_managed_eager_go() {
    dharitri_wasm_debug::denali_go("denali/echo_varargs_managed_eager.scen.json");
}

#[test]
fn echo_varargs_managed_sum_go() {
    dharitri_wasm_debug::denali_go("denali/echo_varargs_managed_sum.scen.json");
}

#[test]
fn echo_varags_tuples_go() {
    dharitri_wasm_debug::denali_go("denali/echo_varags_tuples.scen.json");
}

#[test]
fn echo_varargs_u32_go() {
    dharitri_wasm_debug::denali_go("denali/echo_varargs_u32.scen.json");
}

#[test]
fn echo_vec_u8_go() {
    dharitri_wasm_debug::denali_go("denali/echo_vec_u8.scen.json");
}

#[test]
fn events_go() {
    dharitri_wasm_debug::denali_go("denali/events.scen.json");
}

#[test]
fn events_legacy_go() {
    dharitri_wasm_debug::denali_go("denali/events_legacy.scen.json");
}

#[test]
fn get_caller_go() {
    dharitri_wasm_debug::denali_go("denali/get_caller.scen.json");
}

#[test]
fn get_cumulated_validator_rewards_go() {
    dharitri_wasm_debug::denali_go("denali/get_cumulated_validator_rewards.scen.json");
}

#[test]
fn managed_buffer_concat_1_go() {
    dharitri_wasm_debug::denali_go("denali/managed_buffer_concat_1.scen.json");
}

#[test]
fn managed_buffer_concat_2_go() {
    dharitri_wasm_debug::denali_go("denali/managed_buffer_concat_2.scen.json");
}

#[test]
fn managed_buffer_eq_go() {
    dharitri_wasm_debug::denali_go("denali/managed_buffer_eq.scen.json");
}

#[test]
fn managed_buffer_overwrite_go() {
    dharitri_wasm_debug::denali_go("denali/managed_buffer_overwrite.scen.json");
}

#[test]
fn managed_buffer_slice_1_go() {
    dharitri_wasm_debug::denali_go("denali/managed_buffer_slice_1.scen.json");
}

#[test]
fn managed_buffer_slice_2_go() {
    dharitri_wasm_debug::denali_go("denali/managed_buffer_slice_2.scen.json");
}

#[test]
fn managed_buffer_random_go() {
    dharitri_wasm_debug::denali_go("denali/managed_buffer_set_random.scen.json");
}

#[test]
fn managed_vec_address_push_go() {
    dharitri_wasm_debug::denali_go("denali/managed_vec_address_push.scen.json");
}

#[test]
fn managed_vec_biguint_push_go() {
    dharitri_wasm_debug::denali_go("denali/managed_vec_biguint_push.scen.json");
}

#[test]
fn only_owner_go() {
    dharitri_wasm_debug::denali_go("denali/only_owner.scen.json");
}

#[test]
fn out_of_gas_go() {
    dharitri_wasm_debug::denali_go("denali/out_of_gas.scen.json");
}

#[test]
fn panic_go() {
    dharitri_wasm_debug::denali_go("denali/panic.scen.json");
}

#[test]
fn return_codes_go() {
    dharitri_wasm_debug::denali_go("denali/return_codes.scen.json");
}

#[test]
fn sc_properties_go() {
    dharitri_wasm_debug::denali_go("denali/sc_properties.scen.json");
}

#[test]
fn sc_result_go() {
    dharitri_wasm_debug::denali_go("denali/sc_result.scen.json");
}

#[test]
fn storage_addr_go() {
    dharitri_wasm_debug::denali_go("denali/storage_addr.scen.json");
}

#[test]
fn storage_big_int_go() {
    dharitri_wasm_debug::denali_go("denali/storage_big_int.scen.json");
}

#[test]
fn storage_big_uint_go() {
    dharitri_wasm_debug::denali_go("denali/storage_big_uint.scen.json");
}

#[test]
fn storage_bool_go() {
    dharitri_wasm_debug::denali_go("denali/storage_bool.scen.json");
}

#[test]
fn storage_clear_go() {
    dharitri_wasm_debug::denali_go("denali/storage_clear.scen.json");
}

#[test]
fn storage_i64_go() {
    dharitri_wasm_debug::denali_go("denali/storage_i64.scen.json");
}

#[test]
fn storage_load_from_address_go() {
    dharitri_wasm_debug::denali_go("denali/storage_load_from_address.scen.json");
}

#[test]
fn storage_i64_bad_go() {
    dharitri_wasm_debug::denali_go("denali/storage_i64_bad.scen.json");
}

#[test]
fn storage_map1_go() {
    dharitri_wasm_debug::denali_go("denali/storage_map1.scen.json");
}

#[test]
fn storage_map2_go() {
    dharitri_wasm_debug::denali_go("denali/storage_map2.scen.json");
}

#[test]
fn storage_map3_go() {
    dharitri_wasm_debug::denali_go("denali/storage_map3.scen.json");
}

#[test]
fn storage_mapper_linked_list_go() {
    dharitri_wasm_debug::denali_go("denali/storage_mapper_linked_list.scen.json");
}

#[test]
fn storage_mapper_map_go() {
    dharitri_wasm_debug::denali_go("denali/storage_mapper_map.scen.json");
}

#[test]
fn storage_mapper_map_storage_go() {
    dharitri_wasm_debug::denali_go("denali/storage_mapper_map_storage.scen.json");
}

#[test]
fn storage_mapper_queue_go() {
    dharitri_wasm_debug::denali_go("denali/storage_mapper_queue.scen.json");
}

#[test]
fn storage_mapper_set_go() {
    dharitri_wasm_debug::denali_go("denali/storage_mapper_set.scen.json");
}

#[test]
fn storage_mapper_single_value_go() {
    dharitri_wasm_debug::denali_go("denali/storage_mapper_single_value.scen.json");
}

#[test]
fn storage_mapper_token_attributes_go() {
    dharitri_wasm_debug::denali_go("denali/storage_mapper_token_attributes.scen.json");
}

#[test]
fn storage_mapper_vec_go() {
    dharitri_wasm_debug::denali_go("denali/storage_mapper_vec.scen.json");
}

#[test]
fn storage_opt_addr_go() {
    dharitri_wasm_debug::denali_go("denali/storage_opt_addr.scen.json");
}

#[test]
fn storage_reserved_go() {
    dharitri_wasm_debug::denali_go("denali/storage_reserved.scen.json");
}

#[test]
fn storage_u64_go() {
    dharitri_wasm_debug::denali_go("denali/storage_u64.scen.json");
}

#[test]
fn storage_u64_bad_go() {
    dharitri_wasm_debug::denali_go("denali/storage_u64_bad.scen.json");
}

#[test]
fn storage_usize_go() {
    dharitri_wasm_debug::denali_go("denali/storage_usize.scen.json");
}

#[test]
fn storage_usize_bad_go() {
    dharitri_wasm_debug::denali_go("denali/storage_usize_bad.scen.json");
}

#[test]
fn storage_vec_u8_go() {
    dharitri_wasm_debug::denali_go("denali/storage_vec_u8.scen.json");
}

#[test]
fn struct_eq_go() {
    dharitri_wasm_debug::denali_go("denali/struct_eq.scen.json");
}
