// BOJ 6602 [Eeny Meeny Moo]
fn josephus(dp: &mut Vec<Vec<i16>>, n: usize, k: usize) -> i16 {
    if n == 1 { return 0; }
    if k == 1 { return n as i16 - 1; }
    if dp[n][k] != -1 { return dp[n][k]; }
    if n < k {
        dp[n][k] = (josephus(dp, n-1, k) + k as i16) % n as i16;
    } else {
        let r = josephus(dp, n - n/k, k) - (n%k) as i16;
        dp[n][k] = if r < 0 { r + n as i16 } else { r + r/(k as i16 - 1) }
    }
    dp[n][k]
}
pub fn main() { read();
    let mut dp = vec![vec![-1; 666]; 150];
    'x: loop {
        let n = next::<usize>();
        if n == 0 { break; }
        for m in 1.. {
            if josephus(&mut dp, n-1, m) == 0 {
                println!("{}", m);
                continue 'x;
            }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}