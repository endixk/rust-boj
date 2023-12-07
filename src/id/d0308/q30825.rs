// BOJ 30825 [Arithmetic Punch]
// Supported by GitHub Copilot

fn go(a: &[i64], k: i64, x: i64) -> (bool, i64) {
    let mut c = 0;
    let mut x = x;
    for &p in a.iter().rev() {
        if p > x { return (false, 0); }
        c += x - p;
        x -= k;
    }
    (true, c)
}
pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<i64>());
    let a = (0..n).map(|_| next::<i64>()).collect::<Vec<_>>();

    let (mut l, mut r) = (0, 1<<60);
    while l < r {
        let m = (l + r) / 2;
        if go(&a, k, m).0 { r = m; }
        else { l = m + 1; }
    }
    println!("{}", go(&a, k, l).1);
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