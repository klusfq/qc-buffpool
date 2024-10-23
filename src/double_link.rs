use std::ptr::NonNull;

pub type QcTd = Option<NonNull<QcDLnode>>;

pub fn parse_qctd(qc: QcTd) -> Option<i32> {
    unsafe {
       match qc {
           Some(sp) => Some((*sp.as_ptr()).val),
           None => None,
       }
    }
}

#[derive(Debug)]
pub struct QcDLnode {
    prev: QcTd,
    next: QcTd,
    val: i32,
}

#[derive(Debug)]
pub struct QcDoubleLink {
    size: usize,
    head: QcTd,
    tail: QcTd,
}

impl QcDoubleLink {
    pub fn new() -> Self {
        return QcDoubleLink {
            head: None,
            tail: None,
            size: 0,
        };
    }

    pub fn push_back(&mut self, val: i32) -> QcTd {
        unsafe {
            let node = Box::new(QcDLnode::new(val, None, None));
            let pnode = Box::into_raw(node);
            let up = NonNull::new_unchecked(pnode);

            if let Some(ptail) = self.tail {
                (*ptail.as_ptr()).next = Some(up);
                (*up.as_ptr()).prev = Some(ptail);
                self.tail = Some(up);
            } else {
                self.head = Some(up);
                self.tail = Some(up);
            }

            self.size += 1;

            return Some(up);
        }
    }

    pub fn pop_front(&mut self) -> QcTd {
        unsafe {
            let out: QcTd;
            if let Some(phead) = self.head {
                out = Some(phead);
                self.head = (*phead.as_ptr()).next.take();
            } else {
                out = None;
            }

            if let Some(nh) = self.head {
                (*nh.as_ptr()).prev = None;
            } else {
                self.tail = None;
            }

            return out;
        }
    }

    pub fn remove_item(qc: QcTd) {
        unsafe {
            if let Some(q) = qc {
                let prev_node = (*q.as_ptr()).prev;
                let next_node = (*q.as_ptr()).next;

                if let Some(pn) = prev_node {
                    (*pn.as_ptr()).next = (*q.as_ptr()).next.take();
                }
                if let Some(nn) = next_node {
                    (*nn.as_ptr()).prev = (*q.as_ptr()).prev.take();
                }
            }
        }
    }

    pub fn reset_item(qc: QcTd, val: i32) {
        unsafe {
            if let Some(q) = qc {
                (*q.as_ptr()).val = val;
            }
        }
    }

    pub fn report(&self) {
        unsafe {
            if self.size == 0 {
                return;
            }

            let mut th = self.head;
            print!("head|");
            while let Some(p) = th {
                let tp = &*p.as_ptr();
                print!("->{}", tp.val);
                th = tp.next;
            }
            println!();

            let mut tt = self.tail;
            print!("tail|");
            while let Some(p) = tt {
                let tp = &*p.as_ptr();
                print!("->{}", tp.val);
                tt = tp.prev;
            }
            println!();
        }
    }

    pub fn len(&self) -> usize {
        return self.size;
    }
}

impl QcDLnode {
    pub fn new(val: i32, prev: QcTd, next: QcTd) -> Self {
        return QcDLnode {
            prev,
            next,
            val,
        };
    }
}
