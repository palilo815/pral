fn eratosthenes(n: usize) -> Vec<bool> {
    let mut sieve = vec![false; n];
    sieve[0] = true;
    sieve[1] = true;
    sieve.iter_mut().skip(4).step_by(2).for_each(|x| *x = true);
    let mut i = 3;
    while i * i < n {
        if !sieve[i] {
            sieve.iter_mut().skip(i * i).step_by(i * 2).for_each(|x| *x = true);
        }
        i += 2;
    }
    sieve
}
