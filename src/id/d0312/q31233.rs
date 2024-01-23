fn go(a: &[u64], n: usize, mid: u64) -> bool {
    let mut p = vec![0; n];
    for i in 0..n {
        p[i] = if a[i] >= mid { 1 } else { -1 };
        if i > 0 { p[i] += p[i - 1]; }
    }
    let (mut max, mut min) = (p[1], 0);
    for i in 2..n {
        min = min.min(p[i - 2]);
        max = max.max(p[i] - min);
    }
    max > 0
}
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<u64>()).collect::<Vec<_>>();
    let (mut l, mut r) = (0, 1 << 60);
    while l <= r {
        let mid = (l + r) >> 1;
        if go(&a, n, mid) { l = mid + 1; }
        else { r = mid - 1; }
    }
    println!("{}", l - 1);
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