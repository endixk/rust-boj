// BOJ 17646 [Sum of Squares 2 (More Huge)]

fn pow(mut a: u128, mut b: u128, m: u128) -> u128 {
    let mut ret = 1;
    while b > 0 {
        if b & 1 > 0 { ret = ret * a % m; }
        a = a * a % m;
        b >>= 1;
    }
    ret
}
fn miller_rabin(n: u128, a: u128) -> bool {
    if n == a { return true }
    let mut d = n - 1;
    while d & 1 == 0 {
        if pow(a, d, n) == n - 1 { return true }
        d >>= 1;
    }
    let tmp = pow(a, d, n);
    return tmp == n - 1 || tmp == 1;
}

const TP: [u128; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
fn is_prime(n: u128) -> bool {
    if n < 2 { return false }
    for &p in &TP {
        if n == p { return true }
        if !miller_rabin(n, p) { return false }
    }
    true
}

const SEED: u128 = 10644389958200715658;
const XORA: u128 = 4698004950614326291;
const XORC: u128 = 8483434721261131979;
extern "C" { fn random() -> u32; }
struct Random { state: u128, }
impl Random {
    fn new() -> Self {
        Random { state: SEED }
    }
    fn next(&mut self) -> u128 {
        self.state = unsafe { (random() as u128) << 96 | (random() as u128) << 64 | (random() as u128) << 32 | random() as u128 };
        self.state
    }
    fn next_interval(&mut self, l: u128, r: u128) -> u128 {
        l + self.next() % (r - l)
    }
}
static mut RANDOM: Random = Random { state: SEED };
fn static_random() -> &'static mut Random { unsafe { &mut *std::ptr::addr_of_mut!(RANDOM) } }

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 { a } else { gcd(b, a % b) }
}
fn pollard_rho(n: u128, factors: &mut Vec<u128>, rng: &mut Random) {
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
        x = (x * x % n + c) % n;
        y = (y * y % n + c) % n;
        y = (y * y % n + c) % n;
        let abs = if x > y { x - y } else { y - x };
        d = gcd(abs, n);
    }
    pollard_rho(d, factors, rng);
    pollard_rho(n / d, factors, rng);
}

fn count(v: &[u128]) -> Vec<(u128, u8)> {
    let mut ret = Vec::new();
    let mut cnt = 1;
    for i in 1..v.len() {
        if v[i] == v[i - 1] { cnt += 1; }
        else {
            ret.push((v[i - 1], cnt));
            cnt = 1;
        }
    }
    ret.push((v[v.len() - 1], cnt));
    ret
}

// Euler's criterion for quadratic residues
fn qres(a: u128, p: u128) -> bool {
    pow(a, (p - 1) / 2, p) == 1
}

// Find quadratic non-residue of prime p
fn non_qres(p: u128) -> u128 {
    for a in 2..p {
        if !qres(a, p) { return a; }
    }
    0
}

// Tonelli-Shanks algorithm
// finds x where x^2 = p-1 (mod p)
fn shanks(p: u128) -> u128 {
    let n = p - 1;
    let s = n.trailing_zeros();
    let q = n >> s;
    let z = non_qres(p);
    let (mut m, mut c, mut t, mut r) = (s, pow(z, q, p), pow(n, q, p), pow(n, q+1>>1, p));
    loop {
        if t == 0 { return 0; }
        if t == 1 { return r; }
        let mut i = 1;
        while pow(t, 1 << i, p) != 1 { i += 1; }
        let b = pow(c, 1 << (m - i - 1), p);
        (m, c, t, r) = (i, b * b % p, t * b % p * b % p, r * b % p);
    }
}

// find largest integer less than or equal to sqrt(p)
fn sqlb(p: u128) -> u128 {
    let (mut l, mut r) = (1, p);
    while l < r {
        let m = (l + r + 1) / 2;
        if m * m <= p { l = m; } else { r = m - 1; }
    }
    l
}

// sum of 2 squares, given prime p where p = 4k + 1
// returns (x, y) where x^2 + y^2 = p
fn sq2p(p: u128) -> (u128, u128) {
    let x = shanks(p);
    let x = if x < p - x { p - x } else { x };
    let (mut a, mut b, l) = (p, x, sqlb(p));
    while b > l { (a, b) = (b, a % b); }
    (b, sqlb(p - b * b))
}

// Lagrange's identity
// (a^2 + b^2)(c^2 + d^2) = (ac + bd)^2 + (ad - bc)^2
fn lagrange(a: u128, b: u128, c: u128, d: u128) -> (u128, u128) {
    return if a * c > b * d {
        (a * d + b * c, a * c - b * d)
    } else if a * d > b * c {
        (a * c + b * d, a * d - b * c)
    } else if b * d > a * c {
        (b * c + a * d, b * d - a * c)
    } else {
        (b * d + a * c, b * c - a * d)
    }
}

// sum of 2 squares, given a number than can be represented as sum of 2 squares
// returns (x, y) where x^2 + y^2 = n
fn sq2(n: u128) -> (u128, u128) {
    let mut factors = Vec::new();
    pollard_rho(n, &mut factors, static_random());

    let mut ret = (1, 0);
    let mut rf = Vec::new();
    for &f in factors.iter() {
        if f % 4 == 3 { ret.0 *= f; }
        else { rf.push(f); }
    }
    let k = sqlb(ret.0);
    if k * k != ret.0 { return (0, 0); }
    ret.0 = k;

    for f in rf {
        let (x, y) = if f == 2 { (1, 1) } else { sq2p(f) };
        ret = lagrange(ret.0, ret.1, x, y);
    }
    ret
}

// sum of 3 squares, given a number than can be represented as sum of 3 squares
// returns (x, y, z) where x^2 + y^2 + z^2 = n
fn sq3(n: u128) -> (u128, u128, u128) {
    let mut factors = Vec::new();
    pollard_rho(n, &mut factors, static_random());
    factors.sort_unstable();
    let factors = count(&factors);
    let mut k = 1;
    for (f, c) in factors {
        if c > 1 { k *= f.pow(c as u32 / 2); }
    }
    let n = n / k / k;

    for i in 1.. {
        let (x, y) = sq2(n - i*i);
        if x != 0 { return (i * k, x * k, y * k); }
    }
    (0, 0, 0)
}

pub fn main() { read();
    let n = next::<u128>();
    let k = sqlb(n);
    if k * k == n {
        println!("1\n{}", k);
        return;
    }

    let mut factors = Vec::new();
    pollard_rho(n, &mut factors, static_random());
    factors.sort_unstable();
    let factors = count(&factors);

    let mut flag = true;
    for (f, c) in factors {
        if f % 4 == 3 && c % 2 != 0 {
            flag = false;
            break;
        }
    }
    if flag {
        let (x, y) = sq2(n);
        assert_eq!(x * x + y * y, n);
        println!("2\n{} {}", x, y);
        return;
    }

    let mut m = n;
    let mut a = 0;
    while m % 4 == 0 { m /= 4; a += 1; }
    if m % 8 == 7 {
        let (x, y, z) = sq3(m - 1);
        let (x, y, z, w) = (x << a, y << a, z << a, 1 << a);
        assert_eq!(x * x + y * y + z * z + w * w, n);
        println!("4\n{} {} {} {}", x, y, z, w);
    } else {
        let (x, y, z) = sq3(n);
        assert_eq!(x * x + y * y + z * z, n);
        println!("3\n{} {} {}", x, y, z);
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