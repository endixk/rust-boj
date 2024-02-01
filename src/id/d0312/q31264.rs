// BOJ 31264 [Shooting Practice]
fn go(x: usize, v: &[usize], n: usize, m: usize, a: usize) -> bool {
    if v[0] > x { return false; }
    let (mut i, mut c, mut k) = (0, 0, 0);
    while k < a {
        while i < n && v[i] <= x + k { i += 1; }
        k += v[i-1]; c += 1;
        if c > m { return false; }
    }
    true
}
pub fn main() { read();
    let (n, m, a) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut v = [0; 100001];
    for _ in 0..n { v[next::<usize>()] = 1; }
    let mut c = 0;
    for i in 1..=100000 { if v[i] == 1 { v[c] = i; c += 1; } }

    let (mut l, mut r) = (0, 1<<17);
    while l < r {
        let mid = (l+r)/2;
        if go(mid, &v[..c], c, m, a) { r = mid; }
        else { l = mid+1; }
    }
    println!("{}", l);
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