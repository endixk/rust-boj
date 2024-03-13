// BOJ 31424 [GCD Game]
use std::collections::HashMap;
fn gcd(a: usize, b: usize) -> usize { if b == 0 { a } else { gcd(b, a % b) } }
fn win(dp: &mut Vec<Vec<Option<bool>>>, k: usize, i: usize, n: usize,
       a: &Vec<usize>, map: &HashMap<usize, usize>) -> bool {
    if i == n { return false; }
    if let Some(&v) = dp[map[&k]][i].as_ref() { return v; }
    let mut ret = false;
    let mut c = 0;
    for &x in a {
        if x % k == 0 { c += 1; continue; }
        let g = gcd(k, x);
        if g == 1 { continue; }
        if !win(dp, g, i + 1, n, a, map) { ret = true; break; }
    }
    if !ret && c > i { ret |= !win(dp, k, i + 1, n, a, map); }
    dp[map[&k]][i] = Some(ret);
    ret
}
fn div(n: usize) -> HashMap<usize, usize> {
    let mut f = vec![];
    for i in 1.. {
        if i * i > n { break; }
        if n % i == 0 {
            f.push(i);
            if i * i != n { f.push(n / i); }
        }
    }
    f.sort_unstable();
    f.into_iter().enumerate().map(|(i, x)| (x, i)).collect()
}
pub fn main() { read();
    let (n, x) = (next::<usize>(), next::<usize>());
    let a = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let map = div(x);
    let mut dp = vec![vec![None; n]; map.len()];
    println!("{}", if win(&mut dp, x, 0, n, &a, &map) { "First" } else { "Second" });
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