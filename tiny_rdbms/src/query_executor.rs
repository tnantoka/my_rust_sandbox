use crate::buffer_pool_manager::BufferPoolManager;
use crate::disk_manager::PageId;
use crate::relly::btree;
use crate::relly::btree::{BTree, SearchMode};
use crate::relly::tuple;
use anyhow::Result;

pub type Tuple = Vec<Vec<u8>>;
pub type TupleSlice<'a> = &'a [Vec<u8>];
pub type BoxExecutor<'a> = Box<dyn Executer + 'a>;

pub enum TupleSearchMode<'a> {
    #[allow(dead_code)]
    Start,
    Key(&'a [&'a [u8]]),
}

impl<'a> TupleSearchMode<'a> {
    fn encode(&self) -> SearchMode {
        match self {
            TupleSearchMode::Start => SearchMode::Start,
            TupleSearchMode::Key(tuple) => {
                let mut key = vec![];
                tuple::encode(tuple.iter(), &mut key);
                SearchMode::Key(key)
            }
        }
    }
}

pub trait Executer {
    fn next(&mut self, buffer_manager: &mut BufferPoolManager) -> Result<Option<Tuple>>;
}

pub trait PlanNode {
    fn start(&self, buffer_manager: &mut BufferPoolManager) -> Result<BoxExecutor>;
}

pub struct ExecSeqScan<'a> {
    table_iter: btree::Iter,
    while_cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> Executer for ExecSeqScan<'a> {
    fn next(&mut self, buffer_manager: &mut BufferPoolManager) -> Result<Option<Tuple>> {
        let (pkey_bytes, tuple_bytes) = match self.table_iter.next(buffer_manager)? {
            Some(pair) => pair,
            None => return Ok(None),
        };
        let mut pkey = vec![];
        tuple::decode(&pkey_bytes, &mut pkey);
        if !(self.while_cond)(&pkey) {
            return Ok(None);
        }
        let mut tuple = pkey;
        tuple::decode(&tuple_bytes, &mut tuple);
        Ok(Some(tuple))
    }
}

pub struct SeqScan<'a> {
    pub table_meta_page_id: PageId,
    pub search_mode: TupleSearchMode<'a>,
    pub while_cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> PlanNode for SeqScan<'a> {
    fn start(&self, buffer_manager: &mut BufferPoolManager) -> Result<BoxExecutor> {
        let btree = BTree::new(self.table_meta_page_id);
        let table_iter = btree.search(buffer_manager, self.search_mode.encode())?;
        Ok(Box::new(ExecSeqScan {
            table_iter,
            while_cond: self.while_cond,
        }))
    }
}

pub struct Filter<'a> {
    pub inner_plan: &'a dyn PlanNode,
    pub cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> PlanNode for Filter<'a> {
    fn start(&self, buffer_manager: &mut BufferPoolManager) -> Result<BoxExecutor> {
        let inner_iter = self.inner_plan.start(buffer_manager)?;
        Ok(Box::new(ExecFilter {
            inner_iter,
            cond: self.cond,
        }))
    }
}

pub struct ExecFilter<'a> {
    inner_iter: BoxExecutor<'a>,
    cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> Executer for ExecFilter<'a> {
    fn next(&mut self, buffer_manager: &mut BufferPoolManager) -> Result<Option<Tuple>> {
        loop {
            match self.inner_iter.next(buffer_manager)? {
                Some(tuple) => {
                    if (self.cond)(&tuple) {
                        return Ok(Some(tuple));
                    }
                }
                None => return Ok(None),
            }
        }
    }
}
