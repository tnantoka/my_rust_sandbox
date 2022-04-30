use anyhow::Result;

use crate::buffer_pool_manager::BufferPoolManager;
use crate::disk_manager::PageId;
use crate::relly::btree::BTree;
use crate::relly::tuple;

pub struct SimpleTable {
    pub meta_page_id: PageId,
    pub num_key_elems: usize,
}

impl SimpleTable {
    pub fn create(&mut self, buffer_manager: &mut BufferPoolManager) -> Result<()> {
        let btree = BTree::create(buffer_manager)?;
        self.meta_page_id = btree.meta_page_id;
        Ok(())
    }

    pub fn insert(&self, buffer_manager: &mut BufferPoolManager, record: &[&[u8]]) -> Result<()> {
        let btree = BTree::new(self.meta_page_id);
        let mut key = vec![];
        tuple::encode(record[..self.num_key_elems].iter(), &mut key);
        let mut value = vec![];
        tuple::encode(record[self.num_key_elems..].iter(), &mut value);
        btree.insert(buffer_manager, &key, &value)?;
        Ok(())
    }
}
