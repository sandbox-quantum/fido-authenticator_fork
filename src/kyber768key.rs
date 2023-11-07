pub(crate) use ctap_types::{
    Bytes
};
use crate::{Error, Result};
pub(crate) type SerializedKeyBytes = trussed::types::MessagePQ;

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Key {
    pub kind: Bytes<16>,
    pub material: Bytes<2400>,
 }

impl Key {
    pub fn serialize(&self) -> Result<SerializedKeyBytes> {
        trussed::cbor_serialize_bytes(self).map_err(|_| Error::Other)
    }

    pub fn deserialize(serialized_data: SerializedKeyBytes) -> Result<Self> {
        trussed::cbor_deserialize(&serialized_data).map_err(|_| Error::Other)
    }

    pub fn get_material(&self) -> &Bytes<2400> {
        return &self.material;
    }
}
