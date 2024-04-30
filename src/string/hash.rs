//! mod 2^64 - 1 hashing

#[derive(Clone, Copy, Debug, Default, Eq, Ord)]
struct H(u64);

impl H {
    fn new(x: u64) -> Self {
        Self(x)
    }
    fn get(&self) -> u64 {
        if self.0 == u64::MAX {
            0
        } else {
            self.0
        }
    }
}

impl std::ops::Add for H {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.0 + rhs.0;
        Self(if x < self.0 { x + 1 } else { x })
    }
}

impl std::ops::Sub for H {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 + !rhs.0)
    }
}

impl std::ops::Mul for H {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.0 as u128 * rhs.0 as u128;
        Self(x as u64 + (x >> 64) as u64)
    }
}

impl PartialEq for H {
    fn eq(&self, o: &Self) -> bool {
        self.get() == o.get()
    }
}

impl PartialOrd for H {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        self.get().partial_cmp(&o.get())
    }
}

impl std::hash::Hash for H {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

impl std::fmt::Display for H {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
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
