use super::key::KeyName;
use rsa::RsaPublicKey;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Clone, serde::Serialize)]
pub struct NameMap {
    map: HashMap<KeyName, RsaPublicKey>,
}
pub type NameMapState = Arc<Mutex<NameMap>>;

impl NameMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, name: KeyName, key: RsaPublicKey) -> Option<RsaPublicKey> {
        self.map.insert(name, key)
    }
    pub fn remove(&mut self, name: &str) -> Option<RsaPublicKey> {
        self.map.remove(name)
    }
    pub fn get(&self, name: &str) -> Option<&RsaPublicKey> {
        self.map.get(name)
    }
    pub fn get_mut(&mut self, name: &str) -> Option<&mut RsaPublicKey> {
        self.map.get_mut(name)
    }
    pub fn contains(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }
    pub fn iter<'a>(&'a self) -> std::collections::hash_map::Iter<'a, KeyName, RsaPublicKey> {
        self.map.iter()
    }
    pub fn iter_mut<'a>(
        &'a mut self,
    ) -> std::collections::hash_map::IterMut<'a, KeyName, RsaPublicKey> {
        self.map.iter_mut()
    }
    pub fn keys<'a>(&'a self) -> std::collections::hash_map::Keys<'a, KeyName, RsaPublicKey> {
        self.map.keys()
    }
}
