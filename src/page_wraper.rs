// TODO: this file is a tmp of wrap the page
use std::sync::{Arc, Mutex, Weak};

use crate::page::QcPager;

pub trait QcCaller {
    fn call_remove(&self);
}

#[derive(Debug, Clone)]
pub struct QcPageWraper {
    pg: Arc<Mutex<QcPager>>,
    mgr: Weak<dyn QcCaller>,
}

impl QcPageWraper {
    pub fn new(page: QcPager, mgr: Weak<dyn QcCaller>) -> Self {
        QcPageWraper {
            pg: Arc::new(Mutex::new(page)),
            mgr,
        }
    }

    pub fn inner(&self) -> Arc<Mutex<QcPager>> {
        Arc::clone(&self.pg)
    }

    pub fn weak(&self) -> Weak<Mutex<QcPager>> {
        Arc::downgrade(&self.pg)
    }

    pub fn weak_count(&self) -> usize {
        Arc::weak_count(&self.pg)
    }

    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.pg)
    }
}

impl Drop for QcPageWraper {
    fn drop(&mut self) {
        self.mgr.upgrade().unwrap().call_remove();
    }
}
