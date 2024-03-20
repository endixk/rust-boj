// BOJ 10854 [Divisors]
#[inline] fn mul(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}
fn pow(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut ret = 1;
    while b > 0 {
        if b & 1 > 0 { ret = mul(ret, a, m); }
        a = mul(a, a, m);
        b >>= 1;
    }
    ret
}
fn miller_rabin(n: u64, a: u64) -> bool {
    if n == a { return true }
    let mut d = n - 1;
    while d & 1 == 0 {
        if pow(a, d, n) == n - 1 { return true }
        d >>= 1;
    }
    let tmp = pow(a, d, n);
    return tmp == n - 1 || tmp == 1;
}

const TP: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
fn is_prime(n: u64) -> bool {
    if n < 2 { return false }
    for &p in &TP {
        if n == p { return true }
        if !miller_rabin(n, p) { return false }
    }
    true
}

const SEED: u64 = 14839709780919905126;
const XORA: u64 = 11618625973841399673;
const XORC: u64 = 12090038175212513926;
struct Random { state: u64, }
impl Random {
    fn new() -> Self {
        Random { state: SEED }
    }
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(XORA).wrapping_add(XORC);
        self.state
    }
    fn next_interval(&mut self, l: u64, r: u64) -> u64 {
        l + self.next() % (r - l)
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
fn pollard_rho(n: u64, factors: &mut Vec<u64>, rng: &mut Random) {
    if n == 1 { return; }
    for &p in &TP {
        if n % p == 0 {
            factors.push(p);
            pollard_rho(n / p, factors, rng);
            return;
        }
    }
    if is_prime(n) { factors.push(n); return; }
    let mut x = rng.next_interval(2, n);
    let mut y = x;
    let c = rng.next_interval(2, n);
    let mut d = 1;
    while d == 1 {
        x = (mul(x, x, n) + c) % n;
        y = (mul(y, y, n) + c) % n;
        y = (mul(y, y, n) + c) % n;
        let abs = if x > y { x - y } else { y - x };
        d = gcd(abs, n);
    }
    pollard_rho(d, factors, rng);
    pollard_rho(n / d, factors, rng);
}
pub fn main() { read();
    let n = next::<u64>();
    let mut rng = Random::new();
    let mut factors = Vec::new();
    pollard_rho(n, &mut factors, &mut rng);
    factors.sort_unstable();

    let (mut ret, mut cnt, mut prv) = (1, 1, 0);
    for x in factors {
        if x == prv { cnt += 1; }
        else { ret *= cnt; cnt = 2; prv = x; }
    }
    println!("{}", ret * cnt);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}