use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

use super::bytevec::ByteVec;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub struct KeyName(String);

impl KeyName {
    pub fn parse(s: String) -> Result<Self, String> {
        if s.len() < 3 {
            return Err("keyname too short, minimum length is 3 characters".into());
        }
        if s.len() > 100 {
            return Err("keyname is too long, maximum length is 100 characters".into());
        }
        for c in s.chars() {
            if !is_valid_char(c) {
                return Err(format!("keyname contains invalid character `{c}`"));
            }
        }
        Ok(Self(s))
    }
    pub fn name(&self) -> &str {
        &self.0
    }
}
impl AsRef<str> for KeyName {
    fn as_ref(&self) -> &str {
        self.name()
    }
}
impl TryFrom<String> for KeyName {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}
impl Into<String> for KeyName {
    fn into(self) -> String {
        self.0
    }
}
impl Borrow<str> for KeyName {
    fn borrow(&self) -> &str {
        self.name()
    }
}
impl std::fmt::Display for KeyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn is_valid_char(c: char) -> bool {
    match c {
        '_' | '-' | '.' => true,
        c if c.is_ascii_alphanumeric() => true,
        _ => false,
    }
}

// A lot of this code is copied from the `jsonwebkey` crate.
// I just need the functionality for (de)serializing a very specific
// key type, and the algorithm used (RSA-OAEP-256) is not supported by
// the crate.

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicJwk {
    /// base64 string - must be "AQAB" or "AQAB=="
    pub e: PublicExponent,

    /// base64 string containing p*q
    pub n: ByteVec,

    /// algorithm used - must be "RSA-OAEP-256"
    pub alg: Algorithm,

    /// key type - must be "RSA"
    pub kty: KeyType,

    /// key use - must be "enc"
    #[serde(default, rename = "use")]
    pub key_use: KeyUse,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyUse {
    #[serde(rename = "enc")]
    #[default]
    Enc
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    #[serde(rename = "RSA")]
    #[default]
    Rsa
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Algorithm {
    #[serde(rename = "RSA-OAEP-256")]
    #[default]
    RsaOaep256,
}


const PUBLIC_EXPONENT: u32 = 65537;
const PUBLIC_EXPONENT_B64: &str = "AQAB"; // little-endian, strip zeros
const PUBLIC_EXPONENT_B64_PADDED: &str = "AQABAA==";

/// The standard RSA public exponent, 65537.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicExponent;

impl Serialize for PublicExponent {
    fn serialize<S: serde::ser::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        PUBLIC_EXPONENT_B64.serialize(s)
    }
}

impl<'de> Deserialize<'de> for PublicExponent {
    fn deserialize<D: serde::de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let e = String::deserialize(d)?;
        if e == PUBLIC_EXPONENT_B64 || e == PUBLIC_EXPONENT_B64_PADDED {
            Ok(Self)
        } else {
            Err(serde::de::Error::custom(&format!(
                "public exponent must be {}",
                PUBLIC_EXPONENT
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_name_passes() {
        let names = ["abc123", "1234", "a.b_c-d123"];
        for n in names {
            assert!(
                KeyName::parse(n.into()).is_ok(),
                "failed to parse valid name"
            );
        }
    }
    #[test]
    fn invalid_name_rejects() {
        let names = ["abc=", "(/*abc123", "abc def", "\n \t \r", ":;'\"`~"];
        for n in names {
            assert!(KeyName::parse(n.into()).is_err(), "accepted invalid name");
        }
    }

    fn valid_public_key() -> PublicJwk {
        let n = include_str!("n").parse().unwrap();
        PublicJwk {
            e: PublicExponent,
            alg: Algorithm::RsaOaep256,
            kty: KeyType::Rsa,
            key_use: KeyUse::Enc,
            n
        }
    }
    #[test]
    fn jwk_invalid_algo_fails() {
        let pk = valid_public_key();
        let n = pk.n;
        let key_s = format!(r#"{{
            "kty": "RSA",
            "n": "{n}",
            "e": "AQAB",
            "alg": "invalid",
            "use": "enc"
        }}"#);
        let key = serde_json::from_str::<PublicJwk>(&key_s);
        assert!(key.is_err(), "deserializing invalid algorithm didn't fail");

    }
    #[test]
    fn jwk_invalid_kty_fails() {
        let pk = valid_public_key();
        let n = pk.n;
        let key_s = format!(r#"{{
            "kty": "Ed25519",
            "n": "{n}",
            "e": "AQAB",
            "alg": "RSA-OAEP-256",
            "use": "enc"
        }}"#);
        let key = serde_json::from_str::<PublicJwk>(&key_s);
        assert!(key.is_err(), "deserializing invalid key type didn't fail");

    }
    #[test]
    fn jwk_invalid_e_fails() {
        let pk = valid_public_key();
        let n = pk.n;
        let key_s = format!(r#"{{
            "kty": "RSA",
            "n": "{n}",
            "e": "invalid",
            "alg": "RSA-OAEP-256",
            "use": "enc"
        }}"#);
        let key = serde_json::from_str::<PublicJwk>(&key_s);
        assert!(key.is_err(), "deserializing invalid public exponent didn't fail");

    }
    #[test]
    fn jwk_invalid_use_fails() {
        let pk = valid_public_key();
        let n = pk.n;
        let key_s = format!(r#"{{
            "kty": "RSA",
            "n": "{n}",
            "e": "AQAB",
            "alg": "RSA-OAEP-256",
            "use": "dec"
        }}"#);
        let key = serde_json::from_str::<PublicJwk>(&key_s);
        assert!(key.is_err(), "deserializing invalid key use didn't fail");

    }
    #[test]
    fn valid_jwk_passes() {
        let pk = valid_public_key();
        let n = pk.n;
        let key_s = format!(r#"{{
            "kty": "RSA",
            "n": "{n}",
            "e": "AQAB",
            "alg": "RSA-OAEP-256",
            "use": "enc"
        }}"#);
        let key = serde_json::from_str::<PublicJwk>(&key_s).expect("failed parsing valid jwk");
        assert_eq!(key.n, n);

    }

    #[test]
    fn serializes_correctly() {
        use serde_json::Value;
        let key = valid_public_key();
        let json = serde_json::to_value(key).expect("failed serializing valid jwk");
        let obj = json.as_object().expect("failed converting jwk json to object");
        assert_eq!(obj["kty"], Value::String("RSA".into()));
        assert_eq!(obj["e"], Value::String("AQAB".into()));
        assert_eq!(obj["alg"], Value::String("RSA-OAEP-256".into()));
        assert_eq!(obj["use"], Value::String("enc".into()));
    }
}
