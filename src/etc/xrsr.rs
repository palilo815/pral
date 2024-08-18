//! Xor-Rotate-Shift-Roate 128+

struct Xrsr {
    s: [usize; 2],
}

impl Xrsr {
    fn new() -> Self {
        let pal = Box::into_raw(Box::new("pal")) as usize;
        let ilo = Box::into_raw(Box::new("ilo")) as usize;
        Self { s: [pal, ilo] }
    }
    fn gen(&mut self) -> usize {
        let s0 = self.s[0];
        let mut s1 = self.s[1];
        let result = s0 + s1;
        s1 ^= s0;
        self.s[0] = s0.rotate_left(24) ^ s1 ^ (s1 << 16);
        self.s[1] = s1.rotate_left(37);
        result
    }
    fn shuffle<T>(&mut self, data: &mut [T]) {
        for i in (1..data.len()).rev() {
            let j = self.gen() % (i + 1);
            data.swap(i, j);
        }
    }
}

#[test]
fn test() {}
