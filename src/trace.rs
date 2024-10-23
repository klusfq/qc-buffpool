use std::collections::HashMap;
use crate::double_link::{parse_qctd, QcDoubleLink, QcTd};

type PageId = u8;

#[derive(Debug)]
pub struct QcTracer {
    dblink: QcDoubleLink,
    pmap: HashMap<PageId, QcTd>,
}

impl QcTracer {
    const MAX_SIZE: usize = 4;

    pub fn new() -> Self {
        QcTracer {
            dblink: QcDoubleLink::new(),
            pmap: HashMap::new(),
        }
    }

    pub fn insert(&mut self, page_id: u8) -> Option<()> {
        let vlen = self.len();

        if vlen >= Self::MAX_SIZE {
            let Some(vv) = self.victim() else {
                return None;
            };

            println!("victim: {vv}");
        }

        // move to back if exist
        if let Some(&pkv) = self.pmap.get(&page_id) {
            QcDoubleLink::remove_item(pkv);
            self.pmap.remove(&page_id);
        }

        let qc = self.dblink.push_back(page_id.into());
        self.pmap.insert(page_id, qc);

        return Some(());
    }

    pub fn victim(&mut self) -> Option<u8> {
        let Some(ov)= parse_qctd(self.dblink.pop_front()) else {
            return None;
        };

        let pid = ov as u8;
        self.pmap.remove(&pid);
        return Some(pid);
    }

    pub fn len(&self) -> usize {
        return self.dblink.len();
    }

    pub fn report(&self) {
        self.dblink.report();
        // println!("double link: {:?}", self.dblink);
        println!("hash map: {:?}", self.pmap);
    }
}
