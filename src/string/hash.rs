//! mod 2^64 - 1 hashing

#[derive(Clone, Copy, Default)]
struct ZeroHasher(u64);

impl std::hash::Hasher for ZeroHasher {
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
    fn write(&mut self, _: &[u8]) {
        unimplemented!()
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

impl std::hash::BuildHasher for ZeroHasher {
    type Hasher = Self;
    fn build_hasher(&self) -> Self::Hasher {
        Self::default()
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct H(u64);

impl H {
    const fn new(x: u64) -> Self {
        Self(x)
    }
    const fn get(&self) -> u64 {
        if self.0 == u64::MAX {
            0
        } else {
            self.0
        }
    }
}

impl std::ops::Add for H {
    type Output = H;
    fn add(self, rhs: Self) -> Self::Output {
        let (r, flow) = self.0.overflowing_add(rhs.0);
        H(r + flow as u64)
    }
}

impl std::ops::Sub for H {
    type Output = H;
    fn sub(self, rhs: Self) -> Self::Output {
        let (r, flow) = self.0.overflowing_sub(rhs.0);
        H(r - flow as u64)
    }
}

impl std::ops::Mul for H {
    type Output = H;
    fn mul(self, rhs: Self) -> Self::Output {
        let r = self.0 as u128 * rhs.0 as u128;
        H(r as u64) + H((r >> 64) as u64)
    }
}

impl std::cmp::PartialEq for H {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl std::cmp::Eq for H {}

impl std::cmp::Ord for H {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get().cmp(&other.get())
    }
}

impl std::cmp::PartialOrd for H {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for H {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl std::hash::Hash for H {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.get())
    }
}

#[test]
fn comparision_operators() {
    let x = H::default();
    let y = H(1);
    let z = H::new(u64::MAX);
    assert_ne!(x, y);
    assert_ne!(y, z);
    assert_eq!(z, x);

    let mut a = [x, H(2), y, z, H(3)];
    a.sort_unstable();
    assert_eq!(a, [H(0), H(0), H(1), H(2), H(3)]);
}

#[test]
fn arithmetic_operators() {
    let x = H(100);
    let y = H(u64::MAX - 100);
    let z = H(u64::MAX - 99);
    assert_eq!(x + y, H(0));
    assert_eq!(x + z, H(1));

    let x = H(100);
    let y = H(101);
    assert_eq!(x - x, H(0));
    assert_eq!(x - y, H(u64::MAX - 1));

    const B_64: u64 = 1_000_000_000;
    const B_128: u128 = 1_000_000_000;
    let x = H(B_64);
    assert_eq!((x * x).get(), B_64 * B_64);
    assert_eq!((x * x * x).get(), (B_128 * B_128 * B_128 % u64::MAX as u128) as u64);
}

#[test]
fn hash() {
    let mut set = std::collections::HashSet::new();
    assert_eq!(set.insert(H(0)), true);
    assert_eq!(set.insert(H(u64::MAX)), false);
    assert_eq!(set.insert(H(1)), true);
}
