// BOJ 30846 [A Million Scarlet Roses]
// Supported by GitHub Copilot

const MAX: usize = 1_000_001;
const MOD: usize = 1_000_000_007;
pub fn main() { read();
    let n = next::<usize>();
    let mut pr = vec![0; MAX];
    let mut dp = vec![0; MAX];
    dp[0] = 1;
    for i in 1..=n {
        let x = next::<usize>();
        dp[i] = (dp[i-1] * 2 - if pr[x] > 0 { dp[pr[x] - 1] } else { 0 } + MOD) % MOD;
        pr[x] = i;
    }
    println!("{}", (dp[n] - 1 + MOD) % MOD);
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