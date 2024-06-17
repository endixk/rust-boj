// BOJ 25779 [Which Number]
fn count(x: i128, v: &Vec<i128>, k: usize) -> i128 {
    let mut ret = 0;
    for bit in 1u32..1<<k {
        let p = (0..k).fold(1, |acc, i| acc * if bit & 1<<i != 0 { v[i] } else { 1 });
        ret += x / p * if bit.count_ones() % 2 == 1 { 1 } else { -1 };
    }
    ret
}
pub fn main() { read();
    let (n, k) = (next::<i128>(), next::<usize>());
    let v = (0..k).map(|_| next::<i128>()).collect::<Vec<_>>();
    let (mut l, mut r) = (1, 1<<64);
    while l < r {
        let m = (l + r) / 2;
        if m - count(m, &v, k) < n { l = m + 1; }
        else { r = m; }
    }
    println!("{}", l);
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