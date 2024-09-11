fn sieve(n: usize) -> Vec<u64> {
    let mut p = vec![true; n+1];
    p[0] = false;
    p[1] = false;
    for i in 2..=n {
        if p[i] {
            for j in (i*i..=n).step_by(i) {
                p[j] = false;
            }
        }
    }
    p.iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i as u64).collect()
}
fn phi_prime(p: u64, pow: u64) -> u64 {
    let mut ret = 1;
    for _ in 0..pow {
        ret *= p;
    }
    ret - ret / p
}
fn phi(x: u64, pv: &[u64]) -> u64 {
    let (mut r, mut x) = (1, x);
    for &p in pv {
        if p * p > x { break; }
        let mut pow = 0;
        while x % p == 0 { pow += 1; x /= p; }
        r *= phi_prime(p, pow);
    }
    return if x > 1 {
        r * phi_prime(x, 1)
    } else { r };
}