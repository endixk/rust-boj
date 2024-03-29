// BOJ 17425 [Sum of Divisors]
fn sieve(n: usize) -> Vec<u64> {
    let mut isp = vec![true; n + 1];
    let mut p = Vec::new();
    isp[0] = false; isp[1] = false;
    for i in 2..n {
        if i*i > n { break; }
        if isp[i] {
            p.push(i as u64);
            let mut j = i * i;
            while j < n as usize + 1 {
                isp[j] = false;
                j += i;
            }
        }
    }
    p
}
fn f(p: &[u64], mut x: u64) -> u64 {
    let mut ret = 1;
    for &p in p {
        if p*p > x { break; }
        let mut e = 0;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        ret *= (p.pow(e + 1) - 1) / (p - 1);
    }
    if x > 1 { ret *= x + 1; }
    ret
}
pub fn main() { read();
    let p = sieve(1_000_000);
    let mut a = vec![0; 1_000_001];
    for i in 1..1_000_001 {
        a[i] = a[i - 1] + f(&p, i as u64);
    }

    for _ in 0..next() {
        println!("{}", a[next::<usize>()]);
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}