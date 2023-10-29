/// RC4OK is a lightweight high-performance cryptographically strong random
/// number generator based on an improved variant of RC4 stream cipher.
///
/// Note, classic RC4 is considered deprecated. The improved RC4OK scheme is proposed in https://ia.cr/2023/1486.
/// This implementation collects inspiration from reference implementation https://github.com/emercoin/rc4ok/blob/09f0724f/rc4ok.c.
#[derive(Copy, Clone)]
pub struct RC4ok {
    s: [u8; 256],
    j: u32,
    i: u8,
}

impl RC4ok {
    /// Initializes the modified RC4 stream cipher state, given a non-empty secret
    /// key as input. Once initialized the RC4OK stream cipher can be used for
    /// generating arbitrary many pseudo random bytes.
    #[inline(always)]
    pub fn init(key: &[u8]) -> Self {
        debug_assert!(key.len() > 0, "Key must be non-empty !");

        let mut state = Self {
            s: [0u8; 256],
            j: 0,
            i: 0,
        };
        state.ksa(key);

        // Ignore first 256 -bytes output of RC4OK psuedo random number generator.
        let mut dump = [0u8; 256];
        state.prga(&mut dump);
        // This line can be optimized away by the compiler, is that a "security" problem ?
        dump.fill(0x00);

        state
    }

    /// Generates `n` random bytes using the modified RC4 stream cipher,
    /// following description on page 4 of https://ia.cr/2023/1486.
    #[inline(always)]
    pub fn generate(&mut self, out: &mut [u8]) {
        self.prga(out);
    }

    /// Adds `16` -bit entropy into the internal state of the modified RC4 stream cipher,
    /// following description on page 5 of https://ia.cr/2023/1486.
    #[inline(always)]
    pub fn add_entropy(&mut self, entropy: u16) {
        let mut jw1 = (self.j >> 16) as u16;

        jw1 = jw1.rotate_left(1);
        jw1 = jw1.wrapping_add(entropy);

        const MASK: u32 = 0x0000ffff;
        self.j = (self.j & MASK) | ((jw1 as u32) << 16);
    }

    /// Given already initialized (and probably used too) RC4OK stream cipher object,
    /// this routine can be invoked for reinitializing it with different key.
    #[inline(always)]
    pub fn reset(&mut self, key: &[u8]) {
        debug_assert!(key.len() > 0, "Key must be non-empty !");
        *self = Self::init(key);
    }

    /// Key Scheduling Algorithm
    ///
    /// Based on classic RC4 stream cipher's key scheduling algorithm.
    /// This improved version of KSA is described on page 5 of https://ia.cr/2023/1486.
    #[inline(always)]
    fn ksa(&mut self, key: &[u8]) {
        let klen = key.len();

        let mut i = 0;
        let mut j = 0;

        while i < 256 {
            j = (j + 233) % 256;
            self.s[i] = j as u8;
            i += 1;
        }

        j = 0;
        i = 0;

        let br = [256, klen];
        let n = br[(klen > 256) as usize];
        let mut k = 0;

        while k < n {
            j = (j
                + self.s[i] as usize
                + key[k % klen // If key byte length is power of 2 (non-zero), modulo division is a trivial operation !
                ] as usize)
                % 256;

            // swap(self.s[i], self.s[j])
            //
            // Possible scenario, i == j, hence cache them in a, b; perform swap; write back.
            let mut a = self.s[i];
            let mut b = self.s[j];

            a ^= b;
            b ^= a;
            a ^= b;

            self.s[i] = a;
            self.s[j] = b;

            i = (i + 1) % 256;
            k += 1;
        }

        self.i = self.s[j ^ 85];
        self.j = 0;
    }

    /// Pseudo Random Generation Algorithm
    ///
    /// Modified PRG algorithm, based on classic RC4 stream cipher's randomness generation
    /// algorithm, desceibed on page 4 of https://ia.cr/2023/1486.
    #[inline(always)]
    fn prga(&mut self, out: &mut [u8]) {
        let mut idx = 0;
        let olen = out.len();

        while idx < olen {
            self.i = self.i.wrapping_add(11);
            let mut a = self.s[self.i as usize];

            self.j = self.j.rotate_left(1).wrapping_add(a as u32);

            let j0 = self.j as u8;
            let mut b = self.s[j0 as usize];

            // swap(self.s[self.i], self.s[j0])
            //
            // Possible scenario, self.i == j0, hence cache them in a, b; perform swap; write back.
            a ^= b;
            b ^= a;
            a ^= b;

            self.s[self.i as usize] = a;
            self.s[j0 as usize] = b;

            let u = a.wrapping_add(b);
            out[idx] = self.s[u as usize];

            idx += 1;
        }
    }
}
