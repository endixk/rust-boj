// BOJ 6574 [Advanced Fruits]
fn lcs(s: &[u8],  t: &[u8]) -> Vec<u8> {
    let (n, m) = (s.len(), t.len());
    let mut dp = vec![vec![0; m+1]; n+1];
    let mut tr = vec![vec![0; m+1]; n+1];
    for i in 0..n { for j in 0..m {
        if s[i] == t[j] {
            dp[i+1][j+1] = dp[i][j] + 1;
            tr[i+1][j+1] = 1;
        } else if dp[i][j+1] < dp[i+1][j] {
            dp[i+1][j+1] = dp[i+1][j];
            tr[i+1][j+1] = 2;
        } else {
            dp[i+1][j+1] = dp[i][j+1];
            tr[i+1][j+1] = 3;
        }
    }}

    let mut ans = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 && j > 0 {
        match tr[i][j] {
            1 => { ans.push(s[i-1]); i -= 1; j -= 1; }
            2 => { j -= 1; }
            3 => { i -= 1; }
            _ => unreachable!()
        }
    }

    ans.reverse();
    ans
}

pub fn main() { read();
    while peek() {
        let (s, t) = (next::<String>().into_bytes(), next::<String>().into_bytes());
        let l = lcs(&s, &t);
        let (mut i, mut j, mut k) = (0, 0, 0);
        while k < l.len() {
            while i < s.len() && s[i] != l[k] { print!("{}", s[i] as char); i += 1; }
            while j < t.len() && t[j] != l[k] { print!("{}", t[j] as char); j += 1; }
            print!("{}", l[k] as char);
            i += 1; j += 1; k += 1;
        }
        while i < s.len() { print!("{}", s[i] as char); i += 1; }
        while j < t.len() { print!("{}", t[j] as char); j += 1; }
        println!();
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<std::iter::Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }