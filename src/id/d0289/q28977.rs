// BOJ 28977 [For Collectivism!]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let mut a = (0..n).map(|i| (next::<usize>(), i+1)).collect::<Vec<_>>();
    a.sort_unstable();

    let mut p = vec![0; n+1]; p[1] = a[0].0;
    for i in 1..n { p[i+1] = p[i] + a[i].0; }

    let (mut ans, mut x) = (1, 1);
    for i in 1..=n {
        let (mut l, mut r) = (i, n+1);
        while l < r {
            let m = (l + r) / 2;
            if p[m] - p[i-1] <= k+(m+1-i)*a[i-1].0 { l = m + 1; }
            else { r = m; }
        }
        let k = l - 1;
        if k+1-i > ans { ans = k+1-i; x = i; }
    }

    println!("{}", n - ans);
    let mut v = vec![];
    for i in 0..x-1 { v.push(a[i].1); }
    for i in x+ans-1..n { v.push(a[i].1); }
    v.sort_unstable();
    for i in v { print!("{} ", i); }
    println!();
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