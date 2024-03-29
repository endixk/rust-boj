// BOJ 30521 [Mike Sees the Storm]
const MOD: i64 = 1_000_000_007;
const MAX: usize = 2_000_001;
static mut GEN: bool = false;
static mut FMOD: [i64; MAX] = [1; MAX];
static mut FINV: [i64; MAX] = [1; MAX];
unsafe fn gen() {
    let mut dp = vec![1; MAX];
    for i in 2..MAX {
        dp[i] = -(MOD / i as i64) * dp[MOD as usize % i] % MOD;
        FMOD[i] = FMOD[i-1] * i as i64 % MOD;
        FINV[i] = FINV[i-1] * dp[i] % MOD;
    }
}
fn ncr(n: usize, r: usize) -> i64 { unsafe { if !GEN { gen(); GEN = true } FMOD[n] * FINV[r] % MOD * FINV[n-r] % MOD  } }
fn pow(mut n: i64, mut p: i64) -> i64 {
    let mut ret = 1;
    while p > 0 {
        if p & 1 == 1 { ret = ret * n % MOD }
        n = n * n % MOD;
        p >>= 1;
    }
    ret
}
pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let mut ans = pow(n as i64, k as i64);
    for i in 0..n {
        let mut p = (ncr(n<<1, n-i) - ncr(n<<1, n-i-1) + 2*MOD) % MOD;
        p = p * pow(i as i64, k as i64) % MOD;
        ans = (ans + p) % MOD;
    }
    println!("{}", ans);
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