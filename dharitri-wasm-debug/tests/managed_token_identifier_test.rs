use dharitri_wasm::types::{BoxedBytes, TokenIdentifier};
use dharitri_wasm_debug::{check_managed_top_decode, check_managed_top_encode_decode, DebugApi};

#[test]
fn test_moax() {
    let _ = DebugApi::dummy();
    assert!(TokenIdentifier::<DebugApi>::moax().is_moax());
}

#[test]
fn test_codec() {
    let api = DebugApi::dummy();
    check_managed_top_encode_decode(
        api.clone(),
        TokenIdentifier::<DebugApi>::moax(),
        TokenIdentifier::<DebugApi>::MOAX_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &TokenIdentifier::<DebugApi>::MOAX_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        api.clone(),
        vec![TokenIdentifier::<DebugApi>::moax()],
        expected.as_slice(),
    );

    // also allowed
    assert_eq!(
        TokenIdentifier::<DebugApi>::moax(),
        check_managed_top_decode::<TokenIdentifier<DebugApi>>(api.clone(), &[])
    );
    assert_eq!(
        vec![TokenIdentifier::<DebugApi>::moax()],
        check_managed_top_decode::<Vec<TokenIdentifier<DebugApi>>>(api, &[0, 0, 0, 0])
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_dct_identifier() {
    let _ = DebugApi::dummy();

    // valid identifier
    assert!(TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC-6258d2"[..]).is_valid_dct_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC123-6258d2"[..]).is_valid_dct_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::<DebugApi>::from_dct_bytes(&b"12345-6258d2"[..]).is_valid_dct_identifier());

    // missing dash
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC6258d2"[..]).is_valid_dct_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"AL-C6258d2"[..]).is_valid_dct_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"alc-6258d2"[..]).is_valid_dct_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC-6258D2"[..]).is_valid_dct_identifier());

    // too many random chars
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALC-6258d2ff"[..]).is_valid_dct_identifier());

    // ticker too short
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"AL-6258d2"[..]).is_valid_dct_identifier());

    // ticker too long
    assert!(!TokenIdentifier::<DebugApi>::from_dct_bytes(&b"ALCCCCCCCCC-6258d2"[..]).is_valid_dct_identifier());
}
