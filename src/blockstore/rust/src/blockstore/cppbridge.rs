use anyhow::{bail, Result};
use rand::{thread_rng, Rng};
use std::fmt::{write, Display, Formatter};
use std::hash::{Hash, Hasher};

use super::{
    encrypted::EncryptedBlockStore,
    inmemory::InMemoryBlockStore,
    BlockStore2,
};
use crate::crypto::symmetric::{Cipher, Aes256Gcm, EncryptionKey};

const BLOCKID_LEN: usize = 16;

#[cxx::bridge]
mod ffi {
    #[namespace = "blockstore::rust::bridge"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct BlockId {
        id: [u8; 16],
    }

    #[namespace = "blockstore::rust::bridge"]
    extern "Rust" {
        type OptionData;
        fn has_value(&self) -> bool;
        fn value(&self) -> Result<&[u8]>;
    }

    #[namespace = "blockstore::rust::bridge"]
    extern "Rust" {
        type RustBlockStore2Bridge;
        fn try_create(&self, id: &BlockId, data: &[u8]) -> Result<bool>;
        fn remove(&self, id: &BlockId) -> Result<bool>;
        fn load(&self, id: &BlockId) -> Result<Box<OptionData>>;
        fn store(&self, id: &BlockId, data: &[u8]) -> Result<()>;
        fn num_blocks(&self) -> Result<u64>;
        fn estimate_num_free_bytes(&self) -> Result<u64>;
        fn block_size_from_physical_block_size(&self, block_size: u64) -> u64;
        fn all_blocks(&self) -> Result<Vec<BlockId>>;

        fn new_inmemory_blockstore() -> Box<RustBlockStore2Bridge>;
        fn new_encrypted_inmemory_blockstore() -> Box<RustBlockStore2Bridge>;
    }
}

pub use ffi::BlockId;

impl BlockId {
    pub fn new_random() -> Self {
        let mut result = Self {
            id: [0; BLOCKID_LEN],
        };
        let mut rng = thread_rng();
        rng.fill(&mut result.id);
        result
    }
    pub fn data(&self) -> &[u8; BLOCKID_LEN] {
        &self.id
    }
}

pub struct OptionData(Option<Vec<u8>>);

impl OptionData {
    fn has_value(&self) -> bool {
        self.0.is_some()
    }

    fn value(&self) -> Result<&[u8]> {
        match &self.0 {
            None => bail!("OptionData doesn't have a value"),
            Some(data) => Ok(data),
        }
    }
}

struct RustBlockStore2Bridge(Box<dyn BlockStore2>);

impl RustBlockStore2Bridge {
    fn try_create(&self, id: &BlockId, data: &[u8]) -> Result<bool> {
        self.0.try_create(id, data)
    }
    fn remove(&self, id: &BlockId) -> Result<bool> {
        self.0.remove(id)
    }
    fn load(&self, id: &BlockId) -> Result<Box<OptionData>> {
        Ok(Box::new(OptionData(self.0.load(id)?)))
    }
    fn store(&self, id: &BlockId, data: &[u8]) -> Result<()> {
        self.0.store(id, data)
    }
    fn num_blocks(&self) -> Result<u64> {
        self.0.num_blocks()
    }
    fn estimate_num_free_bytes(&self) -> Result<u64> {
        self.0.estimate_num_free_bytes()
    }
    fn block_size_from_physical_block_size(&self, block_size: u64) -> u64 {
        self.0.block_size_from_physical_block_size(block_size)
    }
    fn all_blocks(&self) -> Result<Vec<BlockId>> {
        Ok(self.0.all_blocks()?.collect())
    }
}

fn new_inmemory_blockstore() -> Box<RustBlockStore2Bridge> {
    Box::new(RustBlockStore2Bridge(Box::new(InMemoryBlockStore::new())))
}

fn new_encrypted_inmemory_blockstore() -> Box<RustBlockStore2Bridge> {
    let key = EncryptionKey::from_hex("9726ca3703940a918802953d8db5996c5fb25008a20c92cb95aa4b8fe92702d9").unwrap();
    Box::new(RustBlockStore2Bridge(Box::new(EncryptedBlockStore::new(
        InMemoryBlockStore::new(),
        Aes256Gcm::new(key),
    ))))
}