pub fn main() { read();
    let s = next::<usize>();
    const MOD: usize = 998_244_353;
    let mut dp = vec![0; 8];
    dp[0] = 6; dp[1] = 1;
    for _ in 2..=s {
        let mut tp = vec![0; 8];
        tp[7] = (dp[7] * 7 + dp[6]) % MOD;
        tp[6] = (dp[6] * 6 + dp[5]) % MOD;
        tp[5] = (dp[5] * 6 + dp[4]) % MOD;
        tp[4] = (dp[4] * 6 + dp[3]) % MOD;
        tp[3] = (dp[3] * 6 + dp[2]) % MOD;
        tp[2] = (dp[2] * 6 + dp[1]) % MOD;
        tp[1] = (dp[1] * 6 + dp[0]) % MOD;
        tp[0] = (dp[0] * 6) % MOD;
        dp = tp;
    }
    println!("{}", dp[7]);
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