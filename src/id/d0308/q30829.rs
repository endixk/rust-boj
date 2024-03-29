// BOJ 30829 [Subsequence MEX]
// Supported by GitHub Copilot

pub fn main() { read();
    let s = next::<String>();
    let s = s.as_bytes();

    let mut k = vec![s.len(); 10];
    let mut dp = vec![0; s.len() + 1];
    let mut tr = vec![0; s.len() + 1];
    let mut md = vec![0; s.len() + 1];
    for (i, &c) in s.into_iter().enumerate().rev() {
        let (mut m, mut t, mut d) = (s.len(), 0, 0);
        for j in 0..10 {
            if dp[k[j]] < m { m = dp[k[j]]; t = k[j]; d = j; }
        }
        dp[i] = m + 1;
        tr[i] = t;
        md[i] = d;
        k[c as usize - b'0' as usize] = i;
    }

    let (mut m, mut t, mut d) = (s.len(), 0, 0);
    for i in 1..10 {
        if dp[k[i]] < m { m = dp[k[i]]; t = k[i]; d = i; }
    }
    print!("{}", d);
    while t != s.len() {
        print!("{}", md[t]);
        t = tr[t];
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}