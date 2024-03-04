// BOJ 6552 [California Jones and the Gate to Freedom]
fn nck(n: usize, k: usize, dp: &mut Vec<Vec<usize>>) -> usize {
    if k > n { return 0; }
    if k == 0 || k == n { return 1; }
    if dp[n][k] != 0 { return dp[n][k]; }
    dp[n][k] = nck(n - 1, k - 1, dp) + nck(n - 1, k, dp);
    dp[n][k]
}
fn kth_combination(n: usize, mut k: usize, a: &[i32], dp: &mut Vec<Vec<usize>>) -> Vec<i32> {
    let mut ret = vec![];
    let mut sz = n >> 1;
    for i in 0..n {
        if k < nck(n - i - 1, sz - 1, dp) {
            ret.push(a[i]);
            if sz > 0 { sz -= 1; }
            if sz == 0 { break; }
        } else {
            k -= nck(n - i - 1, sz - 1, dp);
        }
    }
    ret
}
pub fn main() { read();
    let mut dp = vec![vec![0; 33]; 33];
    loop {
        let n = next::<usize>();
        if n == 0 { break; }
        let a = (0..n).map(|_| next::<i32>()).collect::<Vec<_>>();

        for _ in 0..next() {
            let k = usize::from_str_radix(&next::<String>(), 2).unwrap();
            let mut x = kth_combination(n, k, &a, &mut dp);
            let mut y = (0..n>>1).map(|_| next::<i32>()).collect::<Vec<_>>();
            x.sort_unstable(); y.sort_unstable();
            println!("{}", if x == y { "TRUE" } else { "FALSE" });
        }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}