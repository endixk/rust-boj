// BOJ 14335 [Distinct Substrings]
// Supported by GitHub Copilot

pub fn main() { read();
    for _ in 0..next() {
        let s = next::<String>();
        let mut pr = vec![0; 80];
        let mut dp = vec![0i64; s.len() + 1];
        dp[0] = 1;
        for (i, c) in s.chars().enumerate() {
            let k = (c as u8 - b'0') as usize;
            dp[i+1] = dp[i] * 2 - if pr[k] > 0 { dp[pr[k] - 1] } else { 0 };
            pr[k] = i+1;
        }
        println!("{}", dp[s.len()]);
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