use dharitri_codec::dharitri_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};

use super::DctLocalRoleFlags;
use crate as dharitri_wasm;
use crate::{derive::TypeAbi, types::ManagedVecItem};

const DCT_ROLE_NONE: &[u8] = &[];
const DCT_ROLE_LOCAL_MINT: &[u8] = b"DCTRoleLocalMint";
const DCT_ROLE_LOCAL_BURN: &[u8] = b"DCTRoleLocalBurn";
const DCT_ROLE_NFT_CREATE: &[u8] = b"DCTRoleNFTCreate";
const DCT_ROLE_NFT_ADD_QUANTITY: &[u8] = b"DCTRoleNFTAddQuantity";
const DCT_ROLE_NFT_BURN: &[u8] = b"DCTRoleNFTBurn";

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Debug, Copy,
)]
pub enum DctLocalRole {
    None,
    Mint,
    Burn,
    NftCreate,
    NftAddQuantity,
    NftBurn,
}

impl DctLocalRole {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Mint => 1,
            Self::Burn => 2,
            Self::NftCreate => 3,
            Self::NftAddQuantity => 4,
            Self::NftBurn => 5,
        }
    }

    pub fn as_role_name(&self) -> &'static [u8] {
        match self {
            Self::None => DCT_ROLE_NONE,
            Self::Mint => DCT_ROLE_LOCAL_MINT,
            Self::Burn => DCT_ROLE_LOCAL_BURN,
            Self::NftCreate => DCT_ROLE_NFT_CREATE,
            Self::NftAddQuantity => DCT_ROLE_NFT_ADD_QUANTITY,
            Self::NftBurn => DCT_ROLE_NFT_BURN,
        }
    }

    pub fn to_flag(&self) -> DctLocalRoleFlags {
        match self {
            Self::None => DctLocalRoleFlags::NONE,
            Self::Mint => DctLocalRoleFlags::MINT,
            Self::Burn => DctLocalRoleFlags::BURN,
            Self::NftCreate => DctLocalRoleFlags::NFT_CREATE,
            Self::NftAddQuantity => DctLocalRoleFlags::NFT_ADD_QUANTITY,
            Self::NftBurn => DctLocalRoleFlags::NFT_BURN,
        }
    }
}

// TODO: can be done with macros, but I didn't find a public library that does it and is no_std
// we can implement it, it's easy
const ALL_ROLES: [DctLocalRole; 5] = [
    DctLocalRole::Mint,
    DctLocalRole::Burn,
    DctLocalRole::NftCreate,
    DctLocalRole::NftAddQuantity,
    DctLocalRole::NftBurn,
];

impl DctLocalRole {
    pub fn iter_all() -> core::slice::Iter<'static, DctLocalRole> {
        ALL_ROLES.iter()
    }
}

impl From<u8> for DctLocalRole {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Mint,
            2 => Self::Burn,
            3 => Self::NftCreate,
            4 => Self::NftAddQuantity,
            5 => Self::NftBurn,
            _ => Self::None,
        }
    }
}

impl<'a> From<&'a [u8]> for DctLocalRole {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == DCT_ROLE_LOCAL_MINT {
            Self::Mint
        } else if byte_slice == DCT_ROLE_LOCAL_BURN {
            Self::Burn
        } else if byte_slice == DCT_ROLE_NFT_CREATE {
            Self::NftCreate
        } else if byte_slice == DCT_ROLE_NFT_ADD_QUANTITY {
            Self::NftAddQuantity
        } else if byte_slice == DCT_ROLE_NFT_BURN {
            Self::NftBurn
        } else {
            Self::None
        }
    }
}

impl ManagedVecItem for DctLocalRole {
    const PAYLOAD_SIZE: usize = 1;
    const SKIPS_RESERIALIZATION: bool = false; // TODO: might be ok to be true, but needs testing
    type Ref<'a> = Self;

    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        u8::from_byte_reader(reader).into()
    }

    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        <u8 as ManagedVecItem>::to_byte_writer(&self.as_u8(), writer)
    }
}
