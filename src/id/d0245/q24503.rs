// BOJ 24503 [blobfearful]
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

const TP: [u64; 3] = [2, 7, 61];
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
fn fact(dp: &mut Vec<Vec<u64>>, p: u64, mut c: u64) -> u64 {
    if p < 100 && dp[p as usize][c as usize] != 0 { return dp[p as usize][c as usize]; }
    for i in 1.. {
        let mut k = i * p;
        while k % p == 0 {
            k /= p; c -= 1;
            if c == 0 {
                if p < 100 { dp[p as usize][c as usize] = i * p; }
                return i * p;
            }
        }
    }
    unreachable!()
}
pub fn main() { read();
    let (k, q) = (next::<u64>(), next::<usize>());
    let mut rng = Random::new();
    let mut factors = vec![];
    pollard_rho(k, &mut factors, &mut rng);
    factors.sort_unstable();

    let mut dp = vec![vec![0; 64]; 100];
    for _ in 0..q {
        let mut x = next::<u64>();
        let mut f = vec![];
        for &p in &factors {
            if x % p != 0 { f.push(p); }
            else { x /= p; }
        }
        if f.is_empty() {
            print!("1 "); continue;
        }

        let mut fc = vec![(f[0], 1)];
        for i in 1..f.len() {
            if f[i] == fc.last().unwrap().0 {
                fc.last_mut().unwrap().1 += 1;
            } else {
                fc.push((f[i], 1));
            }
        }

        let mut ans = 1;
        for (p, c) in fc {
            ans = ans.max(fact(&mut dp, p, c));
        }
        print!("{} ", ans);
    }
    println!();
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