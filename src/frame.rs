use std::{cell::RefCell, collections::{HashMap, VecDeque}, error::Error, fs::File, io, os::unix::fs::{FileExt, OpenOptionsExt}, path::Path, rc::Rc};

use crate::{error::QcBupoError, page::QcPager};
use crate::trace::QcTracer;

const PG_NUM: u8 = 16;
const FM_NUM: u8 = 4;

pub struct QcBupo {
    tracer: Box<QcTracer>,              // extern replace tracer
    frame: Vec<Rc<RefCell<QcPager>>>,     // frame list
    pchain: VecDeque<u8>,               // waiting fan-out
    pg_tbl: HashMap<u8, (usize, u32)>,  // <page_id> => (<frame_id>, <ref_count>)
    fd: Box<File>,
}

impl QcBupo {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        use std::fs::OpenOptions;

        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .custom_flags(libc::O_DIRECT)
            .open(path.as_ref()).unwrap();

        fd.set_len(4096 * PG_NUM as u64).unwrap();

        let pg = Rc::new(RefCell::new(QcPager::new()));

        QcBupo {
            tracer: Box::new(QcTracer::new()),
            frame: vec![pg;FM_NUM as usize],
            pchain: VecDeque::new(),
            pg_tbl: HashMap::new(),
            fd: Box::new(fd),
        }
    }

    pub fn report(&self) {
        println!("pg_tbl: {:?}", self.pg_tbl);
    }

    // -- 获取page
    pub fn fetch_page(&mut self, page_id: u8) -> Result<Rc<RefCell<QcPager>>, io::Error> {
        if let Some(pinfo) = self.pg_tbl.get_mut(&page_id) {
            // 存在
            pinfo.1 += 1;
            let frame_id = pinfo.0;
            return Ok(Rc::clone(&self.frame[frame_id as usize]));
        } else {
            // 不存在
            let fid = self.enable_frame_id();
            self.pg_tbl.insert(page_id, (fid, 1));

            let result = self.fd.read_at(
                &mut self.frame[fid].borrow_mut().mut_buffer(),
                (page_id as u64 - 1) * 4096,
            );

            return match result {
                Ok(_) => Ok(Rc::clone(&self.frame[fid])),
                Err(e) =>  Err(e),
            };
        }

    }

    // -- unpin
    pub fn unpin_page(&mut self, page_id: u8) -> Option<()> {
        let Some(pinfo) = self.pg_tbl.get_mut(&page_id) else {
            return None;
        };

        pinfo.1 -= 1;

        if pinfo.1 <= 0 {
            self.pchain.push_back(page_id);
        }

        return Some(());
    }

    // -- 刷盘
    pub fn flush_page(&mut self, page_id: u8) -> Result<(), Box<dyn Error>> {
        let Some(&pinfo) = self.pg_tbl.get(&page_id) else {
            return Err(Box::new(QcBupoError));
        };

        let frame_id = pinfo.0;
        let result = self.fd.write_at(
            &self.frame[frame_id].borrow_mut().buffer(),
            (page_id as u64 - 1) * 4096,
        );

        return match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        };
    }

    // -- 可用frame
    fn enable_frame_id(&mut self) -> usize {
        return 3;
        // 存在空位
    //     if let some(frame_id) = self.tracer.empty_id() {
    //         return frame_id;
    //     }

    //     let some(pid) = self.tracer.victim() else {
    //         panic!("conflict the pager tracer");
    //     };

    //     return pid.1;
    }


}
