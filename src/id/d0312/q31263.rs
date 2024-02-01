// BOJ 31263 [Longest Force]
fn valid(s: &[u8], i: usize, j: usize) -> bool {
    if s[i] == b'0' { return false; }
    let k = s[i..=j].iter().fold(0, |acc, &x| acc * 10 + (x - b'0') as usize);
    k <= 641
}
pub fn main() { read();
    let n = next::<usize>();
    let s = next::<String>().into_bytes();
    if n == 1 { println!("1"); return; }

    let mut dp = vec![0; n+1];
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i-1] + 1;
        if valid(&s, i-2, i-1) { dp[i] = dp[i].min(dp[i-2] + 1); }
        if i > 2 && valid(&s, i-3, i-1) { dp[i] = dp[i].min(dp[i-3] + 1); }
    }
    println!("{}", dp[n]);
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