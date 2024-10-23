use crate::page::QcPager;


pub struct QcBuffpool {
}

impl QcBuffpool {
    pub fn fetch_page(&mut self) -> QcPager {
        return QcPager::new();
    }
}
