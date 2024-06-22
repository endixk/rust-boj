// BOJ 15477 [Mizuyokan]
const INF: u32 = 0x3f3f3f3f;
fn get(dp: &mut Vec<Vec<u32>>, p: &Vec<u32>, i: usize, j: usize, min: u32) -> u32 {
    if dp[i][j] != 0 { return dp[i][j]; }
    if p[j] - p[i] < min { dp[i][j] = INF; }
    else if i+1 == j { dp[i][j] = p[j] - p[i]; }
    else {
        let mut ret = p[j] - p[i];
        for k in i+1..j {
            ret = ret.min(get(dp, p, i, k, min).max(get(dp, p, k, j, min)));
        }
        dp[i][j] = ret;
    }
    dp[i][j]
}
pub fn main() { read();
    let n = next::<usize>();
    let mut p = vec![0; n+1];
    for i in 0..n { p[i+1] = p[i] + next::<u32>(); }

    let mut ans = INF;
    for s in 0..n { for e in s+1..=n {
        if s == 0 && e == n { continue; }
        let min = p[e] - p[s];

        let mut dp = vec![vec![0; n+1]; n+1];
        let l = if s == 0 { 0 } else { get(&mut dp, &p, 0, s, min) };
        let r = if e == n { 0 } else { get(&mut dp, &p, e, n, min) };
        ans = ans.min(l.max(r) - min);
    }}

    println!("{}", ans);
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}