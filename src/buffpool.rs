use std::{collections::HashMap, fmt::Error, fs::{File, OpenOptions}, os::unix::fs::{FileExt, OpenOptionsExt}, sync::{Arc, Mutex, Weak}};

use crate::{bitmap::Qcbitmap, error::QcBupoError, page::QcPager, trace::PageId};

#[derive(Debug)]
struct QcBuffItem {
    frame_id: usize,
    ref_num: i32,
}

impl QcBuffItem {
    pub fn new(frame_id: usize, ref_num: i32) -> Self {
        return QcBuffItem{
            frame_id,
            ref_num,
        };
    }
}

pub struct QcBuffpool {
    frame: Vec<Arc<Mutex<QcPager>>>,
    frame_bits: Qcbitmap,
    table: HashMap<PageId, QcBuffItem>,
    storage: Box<File>,
}

impl QcBuffpool {
    pub fn new(size: usize) -> Self {
        use std::fs::OpenOptions;
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("tmp_buffer.db").unwrap();

        fd.set_len(4096 * 16).unwrap();

        let mut bf = Vec::new();
        for _ in 0..size {
            bf.push(Arc::new(Mutex::new(QcPager::new())))
        }

        return QcBuffpool {
            frame: bf,
            frame_bits: Qcbitmap::new(size),
            table: HashMap::new(),
            storage: Box::new(fd),
        };
    }

    // FIXME
    pub fn fetch_page(&mut self, page_id: PageId) -> Weak<Mutex<QcPager>> {
        if let Some(pgi) = self.table.get_mut(&page_id) {
            pgi.ref_num += 1;
            return Arc::downgrade(&self.frame[pgi.frame_id]);
        } else {
            let Some(npgid) = self.frame_bits.issue() else {
                panic!("fill pool");
            };
            self.frame_bits.set(npgid);

            let mut tmp_pg = QcPager::new();

            self.table.insert(page_id, QcBuffItem::new(npgid, 1));
            self.storage.read_at(
                tmp_pg.mut_buffer(), 
                4096 * (page_id as u64),
            ).unwrap();

            if tmp_pg.is_valiable() {
                *(self.frame[npgid].lock().unwrap()) = tmp_pg;
            }

            return Arc::downgrade(&self.frame[npgid]);
        }
    }

    // FIXME
    pub fn flush_page(&mut self, page_id: PageId) -> Result<(), QcBupoError> {
        if let Some(pgi) = self.table.get_mut(&page_id) {
            return if Arc::strong_count(&self.frame[pgi.frame_id]) > 1 {
            // return if pgi.ref_num > 0 {
                Err(QcBupoError)
            } else {
                self.storage.write_at(
                    self.frame[pgi.frame_id].lock().unwrap().buffer(), 
                    4096 * (page_id as u64),
                ).unwrap();

                self.storage.sync_all().unwrap();

                Ok(())
            };
        } else {
            return Ok(());
        }
    }

    pub fn report(&self) {
        // println!("----frame----");
        // for kp in self.frame.iter().enumerate() {
        // }
        println!("---frame bits---");
        self.frame_bits.report();
        println!("---table---");
        for (ti, tb) in self.table.iter() {
            println!("\t{ti} info: ");
            println!("\t{:?}", tb);
        }
        println!("---frame---");
        print!("\t");
        for (ti, kb) in self.frame.iter().enumerate() {
            print!("{}: {} {} -> ", ti, Arc::strong_count(kb), Arc::weak_count(kb));
        }
        println!("|");
    }

}
