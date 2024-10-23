mod double_link;

pub mod error;
pub mod page;
pub mod trace;

pub mod frame;
pub mod buffpool;

#[cfg(test)]
mod tests {
    use double_link::QcDoubleLink;
    use page::QcPager;
    use trace::QcTracer;

    use super::*;

    #[test]
    fn test_pager() {
        let pager = QcPager::new();

        pager.report();
    }

    #[test]
    fn test_pager_insert_search() {
        let mut pager = QcPager::new();

        pager.save(10, "heelo".to_string());
        pager.save(5, "maike".to_string());
        pager.save(5, "lixdt".to_string());
        pager.save(18, "".to_string());
        pager.save(12, "qiuqiu".to_string());
        pager.report();

        println!("--- search ---");
        for sv in [12, 4, 5, 2] {
            let ovx = pager.obtain(sv);
            match ovx {
                Some(v) => println!("obtain: {sv} -> {v}"),
                None => println!("obtain: {sv} -> <>"),
            }
        }

    }

    #[test]
    fn test_double_link() {
        let mut dpk = QcDoubleLink::new();

        dpk.push_back(4);
        dpk.push_back(9);
        let ko = dpk.push_back(2);
        dpk.push_back(18);
        dpk.report();

        QcDoubleLink::reset_item(ko, 1);
        dpk.report();

        // dpk.pop_front();
        QcDoubleLink::remove_item(ko);
        dpk.report();
    }

    #[test]
    fn test_tracer() {
        let mut tracer = QcTracer::new();

        tracer.insert(12);
        tracer.report();

        tracer.insert(10);
        tracer.report();
        tracer.insert(5);
        tracer.report();
        tracer.insert(10);
        tracer.report();
        tracer.insert(11);
        tracer.report();
    }

}
