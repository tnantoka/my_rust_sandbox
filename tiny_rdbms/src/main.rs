mod buffer_pool_manager;
mod disk_manager;
mod query_executor;
mod relly;
mod table;

use anyhow::Result;
use buffer_pool_manager::{BufferPool, BufferPoolManager};
use disk_manager::{DiskManager, PageId};
use query_executor::{Filter, PlanNode, SeqScan, TupleSearchMode};
use relly::btree::{BTree, SearchMode};
use relly::tuple;
use table::SimpleTable;

fn main() -> Result<()> {
    let heap_file_path = "simple.trdms";

    let disk = DiskManager::open(heap_file_path)?;
    let pool = BufferPool::new(10);
    let mut buffer_manager = BufferPoolManager::new(disk, pool);

    let mut table = SimpleTable {
        meta_page_id: PageId::INVALID_PAGE_ID,
        num_key_elems: 1,
    };

    _ = table.create(&mut buffer_manager);

    table.insert(&mut buffer_manager, &[b"z", b"Alice", b"Smith"])?;
    table.insert(&mut buffer_manager, &[b"x", b"Bob", b"Johnson"])?;
    table.insert(&mut buffer_manager, &[b"y", b"Charlie", b"Williams"])?;
    table.insert(&mut buffer_manager, &[b"w", b"Dave", b"Miller"])?;
    table.insert(&mut buffer_manager, &[b"v", b"Eve", b"Brown"])?;

    buffer_manager.flush()?;

    println!("== procedural");
    {
        let disk = DiskManager::open(heap_file_path)?;
        let pool = BufferPool::new(10);
        let mut bufmgr = BufferPoolManager::new(disk, pool);

        let btree = BTree::new(PageId(0));
        let mut iter = btree.search(&mut bufmgr, SearchMode::Start)?;

        while let Some((key, value)) = iter.next(&mut bufmgr)? {
            let mut record = vec![];
            tuple::decode(&key, &mut record);
            tuple::decode(&value, &mut record);
            println!("{:?}", tuple::Pretty(&record));
        }
    }

    println!("== executor");
    {
        let disk = DiskManager::open(heap_file_path)?;
        let pool = BufferPool::new(10);
        let mut bufmgr = BufferPoolManager::new(disk, pool);

        let plan = Filter {
            cond: &|record| record[1].as_slice() < b"Dave",
            inner_plan: &SeqScan {
                table_meta_page_id: PageId(0),
                search_mode: TupleSearchMode::Key(&[b"w"]),
                while_cond: &|pkey| pkey[0].as_slice() < b"z",
            },
        };
        let mut exec = plan.start(&mut bufmgr)?;

        while let Some(record) = exec.next(&mut bufmgr)? {
            println!("{:?}", tuple::Pretty(&record));
        }
    }

    Ok(())
}
