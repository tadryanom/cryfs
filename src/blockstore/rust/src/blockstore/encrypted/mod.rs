use anyhow::{anyhow, Result};
use sysinfo::{System, SystemExt};

use super::{BlockId, BlockStore2};

mod ciphers;

pub use ciphers::{aes_gcm::Aes256Gcm, Cipher, EncryptionKey};

pub struct EncryptedBlockStore<C: Cipher, B: BlockStore2> {
    underlying_block_store: B,
    encryption_key: C::EncryptionKey,
}

impl<C: Cipher, B: BlockStore2> EncryptedBlockStore<C, B> {
    pub fn new(underlying_block_store: B, encryption_key: C::EncryptionKey) -> Self {
        Self {
            underlying_block_store,
            encryption_key,
        }
    }
}

impl<C: Cipher, B: BlockStore2> BlockStore2 for EncryptedBlockStore<C, B> {
    fn try_create(&self, id: &BlockId, data: &[u8]) -> Result<bool> {
        let ciphertext = C::encrypt(data, &self.encryption_key)?;
        self.underlying_block_store.try_create(id, &ciphertext)
    }

    fn remove(&self, id: &BlockId) -> Result<bool> {
        self.underlying_block_store.remove(id)
    }

    fn load(&self, id: &BlockId) -> Result<Option<Vec<u8>>> {
        let loaded = self.underlying_block_store.load(id)?;
        match loaded {
            None => Ok(None),
            Some(ciphertext) => Ok(Some(C::decrypt(&ciphertext, &self.encryption_key)?)),
        }
    }

    fn store(&self, id: &BlockId, data: &[u8]) -> Result<()> {
        let ciphertext = C::encrypt(data, &self.encryption_key)?;
        self.underlying_block_store.store(id, &ciphertext)
    }

    fn num_blocks(&self) -> Result<u64> {
        self.underlying_block_store.num_blocks()
    }

    fn estimate_num_free_bytes(&self) -> Result<u64> {
        self.underlying_block_store.estimate_num_free_bytes()
    }

    fn block_size_from_physical_block_size(&self, block_size: u64) -> u64 {
        block_size
    }

    fn all_blocks(&self) -> Result<Box<dyn Iterator<Item = BlockId>>> {
        self.underlying_block_store.all_blocks()
    }
}
