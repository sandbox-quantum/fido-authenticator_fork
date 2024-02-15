// use core::str::Bytes;

// use core::str::Bytes;

// use ctap_types::{String};

use pqcrypto_dilithium::ffi::PQCLEAN_DILITHIUM3_CLEAN_CRYPTO_SECRETKEYBYTES;

pub(crate) use ctap_types::Bytes;

use crate::{Error, Result};

// pub type Material = Vec<u8 , PQCLEAN_DILITHIUM3_CLEAN_CRYPTO_SECRETKEYBYTES>;
// pub type SerializedKeyBytes = Vec<u8, 4016>;
pub(crate) type SerializedKeyBytes = trussed::types::MessagePQ;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Key {
    pub kind: Bytes<16>,
    pub material: Bytes<PQCLEAN_DILITHIUM3_CLEAN_CRYPTO_SECRETKEYBYTES>,
}

impl Key {
    pub fn serialize(&self) -> Result<SerializedKeyBytes> {
        trussed::cbor_serialize_bytes(self).map_err(|_| Error::Other)
    }

    pub fn deserialize(serialized_data: SerializedKeyBytes) -> Result<Self> {
        trussed::cbor_deserialize(&serialized_data).map_err(|_| Error::Other)
    }

    pub fn get_material(&self) -> &Bytes<PQCLEAN_DILITHIUM3_CLEAN_CRYPTO_SECRETKEYBYTES> {
        &self.material
    }
}
