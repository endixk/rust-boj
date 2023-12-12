// BOJ 30717 [Subway]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, mut p, k) = (next::<usize>(), next::<i64>(), next::<i64>());
    let mut a = (0..n).map(|_| next::<i64>()).collect::<Vec<_>>();
    a.sort_unstable_by(|x, y| (y % k).cmp(&(x % k)));

    let mut ans = 0;
    for i in 0..n {
        let r = k - a[i] % k;
        if p >= r { p -= r; a[i] += r; }
        ans += a[i] / k;
    }
    println!("{}", ans + p / k);
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