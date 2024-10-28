#[derive(Debug)]
pub struct Qcbitmap(Vec<u8>);

impl Qcbitmap {
    pub fn new(size: usize) -> Self {
        let num = (size - 1) / 8 + 1;

        Qcbitmap(vec![0;num])
    }

    pub fn set(&mut self, idx: usize) -> Option<()> {
        let block_num = idx / 8;
        let block_offset = idx % 8;

        self.0[block_num] |= 0b10000000 >> block_offset;

        Some(())
    }

    pub fn issue(&self) -> Option<usize> {
        for (ox, &ob) in self.0.iter().enumerate() {
            let mut ci = 0_usize;
            let mut obt = !ob;
            while ci < 8 {
                if obt & (1_u8 << 7) == 0 {
                    obt <<= 1;
                    ci += 1;
                    continue;
                }

                return Some(ox * 8 + ci);
            }
        }

        return None;
    }

    pub fn report(&self) {
        for &k in self.0.iter() {
            print!("{:#010b}, ", k);
        }
        println!();
    }
}
