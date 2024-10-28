mod double_link;

pub mod error;
pub mod page;
pub mod trace;

// pub mod frame;
pub mod buffpool;
pub mod bitmap;

#[cfg(test)]
mod tests {
    use bitmap::Qcbitmap;
    use buffpool::QcBuffpool;

    use super::*;

    #[test]
    fn test_bitmap() {
        let mut bitm = Qcbitmap::new(10);
        bitm.report();
        bitm.set(0);
        bitm.report();
        bitm.set(1);
        bitm.set(5);
        bitm.report();
        bitm.set(10);
        bitm.report();
        println!("issue: {}", bitm.issue().unwrap());
    }

    // #[test]
    // fn test_pager() {
    //     let pager = QcPager::new();

    //     pager.report();
    // }

    // #[test]
    // fn test_pager_insert_search() {
    //     let mut pager = QcPager::new();

    //     pager.save(10, "heelo".to_string());
    //     pager.save(5, "maike".to_string());
    //     pager.save(5, "lixdt".to_string());
    //     pager.save(18, "".to_string());
    //     pager.save(12, "qiuqiu".to_string());
    //     pager.report();

    //     println!("--- search ---");
    //     for sv in [12, 4, 5, 2] {
    //         let ovx = pager.obtain(sv);
    //         match ovx {
    //             Some(v) => println!("obtain: {sv} -> {v}"),
    //             None => println!("obtain: {sv} -> <>"),
    //         }
    //     }

    // }

    // #[test]
    // fn test_double_link() {
    //     let mut dpk = QcDoubleLink::new();

    //     dpk.push_back(4);
    //     dpk.push_back(9);
    //     let ko = dpk.push_back(2);
    //     dpk.push_back(18);
    //     dpk.report();

    //     QcDoubleLink::reset_item(ko, 1);
    //     dpk.report();

    //     // dpk.pop_front();
    //     QcDoubleLink::remove_item(ko);
    //     dpk.report();
    // }

    // #[test]
    // fn test_tracer() {
    //     let mut tracer = QcTracer::new();

    //     tracer.insert(12);
    //     tracer.report();

    //     tracer.insert(10);
    //     tracer.report();
    //     tracer.insert(5);
    //     tracer.report();
    //     tracer.insert(10);
    //     tracer.report();
    //     tracer.insert(11);
    //     tracer.report();
    // }


    #[test]
    fn test_buffpool() {
        let mut bufpool = QcBuffpool::new(8);
        bufpool.fetch_page(1);
        let pg = bufpool.fetch_page(2).upgrade().unwrap();
        pg.lock().unwrap().save(10, "hsdfp".to_string());
        pg.lock().unwrap().save(5, "klusfq".to_string());
        pg.lock().unwrap().report();
        bufpool.report();
        drop(pg);
        bufpool.report();
        bufpool.flush_page(2).unwrap();

        bufpool.fetch_page(7);
        bufpool.fetch_page(8);
        bufpool.fetch_page(9);
        bufpool.fetch_page(5);
        bufpool.fetch_page(7);
        bufpool.fetch_page(12);
        let kg = bufpool.fetch_page(6).upgrade().unwrap();
        kg.lock().unwrap().save(12, "this ok".to_string());
        drop(kg);
        bufpool.report();
        // bufpool.fetch_page(1);
    }
}
