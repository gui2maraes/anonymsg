use std::borrow::Borrow;

use rsa::pkcs8::DecodePublicKey;
use rsa::pkcs8::EncodePublicKey;
use rsa::RsaPublicKey;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct PemPublicKey(pub rsa::RsaPublicKey);
impl PemPublicKey {
    pub fn pem_string(&self) -> String {
        self.0
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .unwrap()
    }
    pub fn from_pem(pem: &str) -> Result<Self, rsa::pkcs8::spki::Error> {
        Ok(Self(RsaPublicKey::from_public_key_pem(pem)?))
    }
}

impl TryFrom<String> for PemPublicKey {
    type Error = rsa::pkcs8::spki::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_pem(&value)
    }
}

impl Into<String> for PemPublicKey {
    fn into(self) -> String {
        self.pem_string()
    }
}

use serde::{Deserialize, Serialize};
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
}
