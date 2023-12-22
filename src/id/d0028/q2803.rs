// BOJ 2803 [KOŠARE]
// Supported by GitHub Copilot

const MOD: i64 = 1_000_000_007;
fn pmod(mut x: i64, mut p: i64) -> i64 {
    let mut r = 1;
    while p > 0 {
        if p & 1 != 0 { r = r * x % MOD; }
        x = x * x % MOD;
        p >>= 1;
    }
    r
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut dp = vec![0; 1<<m];
    for _ in 0..n {
        let mut bit = 0;
        for _ in 0..next() { bit |= 1 << next::<usize>() - 1; }
        dp[bit] += 1;
    }
    for i in 0..m { for j in 0..1<<m {
        if j & 1 << i != 0 { dp[j] += dp[j ^ 1 << i]; }
    }}
    let mut tp = vec![0; 1<<m];
    for i in 0..1<<m { tp[i ^ (1<<m)-1] = dp[i]; }
    dp = tp.iter().map(|&x| pmod(2, x)).collect();

    let mut ans = 0;
    for i in 0..1usize<<m {
        if i.count_ones() % 2 == 0 { ans = (ans + dp[i]) % MOD; }
        else { ans = (ans - dp[i]) % MOD; }
    }
    println!("{}", (ans + MOD) % MOD);
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