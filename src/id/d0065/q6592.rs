// BOJ 6592 [Compromise]
use std::collections::HashMap;
#[inline] fn djb2(s: &String) -> u64 { s.chars().fold(5381, |h, c| h.wrapping_mul(33).wrapping_add(c as u64)) }
fn lcs(a: Vec<u64>, b: Vec<u64>) -> Vec<u64> {
    let (n, m) = (a.len(), b.len());
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] { dp[i][j] = dp[i - 1][j - 1] + 1; }
            else { dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]); }
        }
    }
    let (mut i, mut j) = (n, m);
    let mut res = vec![];
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            res.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] { i -= 1; }
        else { j -= 1; }
    }
    res.reverse();
    res
}
pub fn main() { read();
    while peek() {
        let mut map = HashMap::new();
        let (mut u, mut v) = (vec![], vec![]);
        loop {
            let s = next::<String>();
            if s == "#" { break; }
            u.push(djb2(&s));
            map.insert(djb2(&s), s);
        }
        loop {
            let s = next::<String>();
            if s == "#" { break; }
            v.push(djb2(&s));
            map.insert(djb2(&s), s);
        }

        let res = lcs(u, v);
        res.into_iter().for_each(|x| print!("{} ", map.get(&x).unwrap()));
        println!();
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, iter::Peekable};
static mut BUF: String = String::new();
static mut IT: Option<Peekable<SplitAsciiWhitespace>> = None;
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
