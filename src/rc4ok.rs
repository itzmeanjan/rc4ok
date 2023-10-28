#[derive(Copy, Clone)]
pub struct Rc4ok {
    s: [u8; 256],
    j: u32,
    i: u8,
}

impl Rc4ok {
    #[inline(always)]
    pub fn init(key: &[u8], klen: usize) -> Self {
        let mut state = Self {
            s: [0u8; 256],
            i: 0,
            j: 0,
        };
        Self::ksa(&mut state, key, klen);
        state
    }

    #[inline(always)]
    fn ksa(state: &mut Self, key: &[u8], klen: usize) {
        let mut i = 0;
        let mut j = 0;

        while i < 256 {
            j = (j + 233) % 256;
            state.s[i] = j as u8;
            i += 1;
        }

        j = 0;
        i = 0;
        while i < 256 {
            j = (j + state.s[i] as usize + key[i % klen] as usize) % 256;

            state.s[i] ^= state.s[j];
            state.s[j] ^= state.s[i];
            state.s[i] ^= state.s[j];

            i += 1;
        }
    }
}
