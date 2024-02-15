use crate::{Error, Result};
pub(crate) use ctap_types::Bytes;
use pqcrypto_kyber::ffi::PQCLEAN_KYBER768_CLEAN_CRYPTO_SECRETKEYBYTES;
pub(crate) type SerializedKeyBytes = trussed::types::MessagePQ;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Key {
    pub kind: Bytes<16>,
    pub material: Bytes<PQCLEAN_KYBER768_CLEAN_CRYPTO_SECRETKEYBYTES>,
}

impl Key {
    pub fn serialize(&self) -> Result<SerializedKeyBytes> {
        trussed::cbor_serialize_bytes(self).map_err(|_| Error::Other)
    }

    pub fn deserialize(serialized_data: SerializedKeyBytes) -> Result<Self> {
        trussed::cbor_deserialize(&serialized_data).map_err(|_| Error::Other)
    }

    pub fn get_material(&self) -> &Bytes<PQCLEAN_KYBER768_CLEAN_CRYPTO_SECRETKEYBYTES> {
        &self.material
    }
}
