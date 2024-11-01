#[derive(Debug, Clone)]
pub struct QcPager {
    dirty: bool,
    data: [u8; 4096],
}

impl QcPager {
    // -- [0~3]: key, [4~5]: pointer, [6~7]: len
    const SLOT_SIZE_LOW: u8 = 8;
    const SLOT_SIZE: usize = Self::SLOT_SIZE_LOW as usize;

    pub fn new() -> Self {
        let mut pg = QcPager {
            dirty: false,
            data: [0_u8; 4096],
        };

        pg.set_slot_len(0);
        pg.set_slot_pointer(16); // -- slot start offset
        pg.set_data_pointer(4095); // -- data end offset

        return pg;
    }

    pub fn buffer(&self) -> &[u8] {
        return self.data.as_slice();
    }

    pub fn mut_buffer(&mut self) -> &mut [u8] {
        self.op_dirty();
        return self.data.as_mut_slice();
    }

    pub fn is_valiable(&self) -> bool {
        self.get_slot_len() > 0
    }

    pub fn save(&mut self, k: u32, v: String) -> Option<usize> {
        let vlen = v.len();
        if vlen > (u16::MAX as usize) {
            return None;
        }

        let slot_len = self.get_slot_len();
        let data_pointer = self.get_data_pointer();

        let (idx, page_opt) = self.binary_search(k);
        if page_opt.is_some() {
            return None;
        };

        self.op_dirty();
        let slot = self.fill_slot(k, data_pointer, vlen as u16);
        self.insert_slot(slot, idx).unwrap();

        self.set_data_pointer(data_pointer - vlen as u16);
        self.set_slot_len(slot_len + Self::SLOT_SIZE_LOW as u16);

        // -- 非空串
        if vlen > 0 {
            let mstar = data_pointer as usize - vlen + 1;
            let m_contain = &mut self.data[mstar..=(data_pointer as usize)];
            m_contain.clone_from_slice(&v.into_bytes());
        }

        return Some(vlen);
    }

    pub fn obtain(&self, k: u32) -> Option<String> {
        let (_, page_opt) = self.binary_search(k);
        let Some(pu) = page_opt else {
            return None;
        };

        let pointer = u16::from_be_bytes(pu[4..6].try_into().unwrap()) as usize;
        let len = u16::from_be_bytes(pu[6..8].try_into().unwrap()) as usize;

        return Some(String::from_utf8(self.data[pointer..(pointer + len)].to_vec()).unwrap());
    }

    pub fn report(&self) {
        let slot_start = self.get_slot_pointer() as usize;
        let slot_len = self.get_slot_len() as usize;

        println!("--- Base data ---");
        println!("slot offset: {}", slot_start);
        println!("slot count: {}", self.count_slot());

        println!("data offset: {}", self.get_data_pointer() as usize);
        // byte array ->> slot array
        let slot_list: Vec<(u32, u16, u16, String)> = self.data
            [slot_start..(slot_start + slot_len)]
            .chunks(Self::SLOT_SIZE)
            .map(|v| <[u8; Self::SLOT_SIZE]>::try_from(v).unwrap())
            .map(|u| {
                (
                    u32::from_be_bytes(u[0..4].try_into().unwrap()),
                    u16::from_be_bytes(u[4..6].try_into().unwrap()),
                    u16::from_be_bytes(u[6..8].try_into().unwrap()),
                )
            })
            .map(|w| {
                (
                    w.0,
                    w.1,
                    w.2,
                    String::from_utf8(self.data[(w.1 as usize)..(w.1 + w.2) as usize].to_vec())
                        .unwrap(),
                )
            })
            .collect();

        println!("slot_list: {:?}", slot_list);
    }

    // -- no repeat key
    //          存在，则返回(idx, <page>)
    //          不存在，则返回(widx, None) - widx为应插入位置
    fn binary_search(&self, key: u32) -> (usize, Option<[u8; Self::SLOT_SIZE]>) {
        let slot_start = self.get_slot_pointer() as usize;
        let slot_len = self.get_slot_len() as usize;

        // byte array ->> slot number array
        let block_list: Vec<u32> = (slot_start..(slot_start + slot_len))
            .step_by(Self::SLOT_SIZE)
            .map(|op| u32::from_be_bytes(self.data[op..(op + 4)].try_into().unwrap()))
            .collect();

        // println!("(binary_search) slot_number_list: {:?}", block_list);
        if block_list.len() == 0 {
            return (0, None);
        }

        let mut pl = 0;
        let mut pr = if block_list.len() > 0 {
            block_list.len() - 1
        } else {
            0
        };
        let mut out = (pl, None);

        // search
        while pl <= pr {
            // println!("(binary_search) ({}, {})", pl, pr);
            let mid = pl + (pr - pl) / 2;
            let neighber = mid == pl;

            if key > block_list[mid] {
                pl = mid;
            } else if key < block_list[mid] {
                pr = mid;
            } else {
                out = (mid, Some(self.idx_slot(mid).unwrap()));
                break;
            }

            if !neighber {
                continue;
            }

            out = if pr == pl {
                (pl, None)
            } else if key < block_list[pr] {
                (pr, None)
            } else if key > block_list[pr] {
                (pr + 1, None)
            } else {
                (pr, Some(self.idx_slot(pr).unwrap()))
            };
            break;
        }

        return out;
    }

    // -- dirty control
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn op_dirty(&mut self) {
        self.dirty = true;
    }
    pub fn op_clear(&mut self) {
        self.dirty = false;
    }

    // -- about slot
    fn set_slot_len(&mut self, len: u16) {
        [self.data[8], self.data[9]] = u16::to_be_bytes(len);
    }
    fn get_slot_len(&self) -> u16 {
        u16::from_be_bytes([self.data[8], self.data[9]])
    }

    fn set_slot_pointer(&mut self, pointer: u16) {
        [self.data[10], self.data[11]] = u16::to_be_bytes(pointer);
    }
    fn get_slot_pointer(&self) -> u16 {
        u16::from_be_bytes([self.data[10], self.data[11]])
    }
    fn idx_slot(&self, idx: usize) -> Option<[u8; Self::SLOT_SIZE]> {
        if (Self::SLOT_SIZE) * idx >= self.get_slot_len() as usize {
            return None;
        }

        let slot_start = self.get_slot_pointer() as usize + (Self::SLOT_SIZE) * idx;
        let mut out = [0_u8; Self::SLOT_SIZE];

        out.copy_from_slice(&self.data[slot_start..(slot_start + Self::SLOT_SIZE)]);

        return Some(out);
    }
    fn fill_slot(&mut self, k: u32, dpointer: u16, vlen: u16) -> [u8; Self::SLOT_SIZE] {
        let mut mslot = [0_u8; Self::SLOT_SIZE];
        let dstart = if vlen > 0 { dpointer - vlen + 1 } else { 0 };

        let inner: Vec<u8> = u32::to_be_bytes(k)
            .into_iter()
            .chain(u16::to_be_bytes(dstart).into_iter())
            .chain(u16::to_be_bytes(vlen).into_iter())
            .collect();

        mslot.clone_from_slice(&inner);

        return mslot;
    }
    fn insert_slot(&mut self, slot: [u8; Self::SLOT_SIZE], idx: usize) -> Option<()> {
        let slot_start = self.get_slot_pointer() as usize;
        let slot_len = self.get_slot_len() as usize;

        // byte array ->> slot array
        let mut slot_list: Vec<[u8; Self::SLOT_SIZE]> = self.data
            [slot_start..(slot_start + slot_len)]
            .chunks(Self::SLOT_SIZE)
            .map(|v| <[u8; Self::SLOT_SIZE]>::try_from(v).unwrap())
            .collect();

        slot_list.splice(idx..idx, vec![slot]);

        self.data[slot_start..(slot_start + slot_len + Self::SLOT_SIZE)]
            .as_mut()
            .clone_from_slice(&slot_list.concat());

        return Some(());
    }

    // -- about data
    fn set_data_pointer(&mut self, pointer: u16) {
        [self.data[12], self.data[13]] = u16::to_be_bytes(pointer);
    }
    fn get_data_pointer(&self) -> u16 {
        u16::from_be_bytes([self.data[12], self.data[13]])
    }

    pub fn count_slot(&self) -> u16 {
        self.get_slot_len() / (Self::SLOT_SIZE_LOW as u16)
    }

    pub fn left_space(&self) -> u16 {
        self.get_data_pointer() - self.get_slot_pointer() - self.get_slot_len()
    }
}
