// BOJ 31565 [Reverse Service]
fn leap(y: u32) -> u32 {
    if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) { 1 } else { 0 }
}
fn ymd(y: u32, m: u32, d: u32) -> u32 {
    let mut r = 0;
    for i in 1990..y { r += 365 + leap(i); }
    for i in 1..m { r += [0, 31, 28 + leap(y), 31, 30, 31, 30, 31, 31, 30, 31, 30][i as usize]; }
    r + d
}
pub fn main() { read();
    let a = ymd(next(), next(), next());
    let b = ymd(next(), next(), next());
    let (t, n) = (next::<usize>(), next::<usize>());

    let mut dp = vec![0; t+1];
    dp[0] = 0;
    for _ in 0..n {
        let (c, v) = match next::<u8>() {
            1 | 2 => (next::<usize>(), next::<u32>()),
            _ => (next::<usize>(), next::<u32>() * 30),
        };
        for i in (c..=t).rev() {
            dp[i] = dp[i].max(dp[i-c] + v);
        }
    }

    let ans = *dp.iter().max().unwrap();
    println!("{}", if b - a > ans { b - a - ans } else { ans + a - b });
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