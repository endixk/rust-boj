// BOJ 4355 [Relatives]
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
pub fn main() { read();
    let pv = sieve(31623);
    loop {
        let n = next::<u64>();
        if n == 0 { break; }
        if n == 1 { println!("0"); continue; }
        println!("{}", phi(n, &pv));
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}