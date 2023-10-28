#[derive(Copy, Clone)]
pub struct Rc4ok {
    s: [u8; 256],
    j: u32,
    i: u8,
}

impl Rc4ok {
    #[inline(always)]
    pub fn init(key: &[u8]) -> Self {
        let mut state = Self {
            s: [0u8; 256],
            i: 0,
            j: 0,
        };
        state.ksa(key);

        let mut dump = [0u8; 256];
        state.prga(&mut dump);

        state
    }

    #[inline(always)]
    pub fn generate(&mut self, out: &mut [u8]) {
        self.prga(out);
    }

    // Pseudo Random Generation Algorithm
    #[inline(always)]
    fn prga(&mut self, out: &mut [u8]) {
        let mut idx = 0;
        let olen = out.len();

        while idx < olen {
            self.i = self.i.wrapping_add(11);
            self.j = self.j.rotate_left(1);

            #[cfg(target_endian = "little")]
            let mut j0 = self.j as u8;
            #[cfg(target_endian = "big")]
            let mut j0 = self.j.swap_bytes() as u8;

            j0 = j0.wrapping_add(self.s[self.i as usize]);

            self.s[self.i as usize] ^= self.s[j0 as usize];
            self.s[j0 as usize] ^= self.s[self.i as usize];
            self.s[self.i as usize] ^= self.s[j0 as usize];

            let u = self.s[self.i as usize].wrapping_add(self.s[j0 as usize]);
            out[idx] = self.s[u as usize];

            idx += 1;
        }
    }

    // Key Scheduling Algorithm
    #[inline(always)]
    fn ksa(&mut self, key: &[u8]) {
        let mut i = 0;
        let mut j = 0;
        let klen = key.len();

        while i < 256 {
            j = (j + 233) % 256;
            self.s[i] = j as u8;
            i += 1;
        }

        j = 0;
        i = 0;
        while i < 256 {
            j = (j + self.s[i] as usize + key[i % klen] as usize) % 256;

            self.s[i] ^= self.s[j];
            self.s[j] ^= self.s[i];
            self.s[i] ^= self.s[j];

            i += 1;
        }

        self.i = self.s[j ^ 85];
        self.j = 0;
    }
}
