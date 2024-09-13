use std::str::FromStr;

use base64::engine::general_purpose::URL_SAFE_NO_PAD as b64;
use base64::Engine;
use serde::{Deserialize, Serialize};

// Byte Vector with base64 (de)serialization
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ByteVec(#[serde(with = "serde_base64")] Vec<u8>);

impl std::fmt::Display for ByteVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&b64.encode(&self.0))
    }
}
impl std::fmt::Debug for ByteVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&b64.encode(&self.0))
    }
}

impl FromStr for ByteVec {
    type Err = base64::DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(b64.decode(s)?))
    }
}
impl From<ByteVec> for String {
    fn from(value: ByteVec) -> Self {
        b64.encode(&value.0)
    }
}

impl<T: Into<Vec<u8>>> From<T> for ByteVec {
    fn from(into_vec: T) -> Self {
        Self(into_vec.into())
    }
}

impl AsRef<[u8]> for ByteVec {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::ops::Deref for ByteVec {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
mod serde_base64 {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD as b64;
    use base64::Engine;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

    pub(crate) fn serialize<S: Serializer>(
        bytes: impl AsRef<[u8]>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        b64.encode(bytes).serialize(s)
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let s = String::deserialize(d)?;
        b64.decode(&s).map_err(|e| {
            #[cfg(debug_assertions)]
            let err_msg = e.to_string().to_lowercase();
            #[cfg(not(debug_assertions))]
            let err_msg = "invalid base64";
            de::Error::custom(err_msg.strip_suffix('.').unwrap_or(&err_msg))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base64_deserialize() {
        let base64_json = r#""MTIzNDU2Nzg5MGFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6""#;
        let bytes = b"1234567890abcdefghijklmnopqrstuvwxyz";
        let bytevec = serde_json::from_str::<ByteVec>(base64_json).expect("failed to deserialize base64");
        assert_eq!(&bytevec[..], bytes);
    }
    #[test]
    fn test_base64_serialize() {
        let base64_json = r#""MTIzNDU2Nzg5MGFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6""#;
        let bytes = b"1234567890abcdefghijklmnopqrstuvwxyz";
        let bytevec = ByteVec::from(bytes);
        let json = serde_json::to_string(&bytevec).expect("failed to serialize bytevec");
        assert_eq!(json, base64_json);
    }

}