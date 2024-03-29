// BOJ 30426 [Rebirth]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, m, k) = (next::<usize>(), next::<usize>(), next::<usize>());
    let (mut g, mut y) = (vec![0; k], vec![0; k]);
    for i in 0..k {
        g[i] = next::<usize>();
        y[i] = next::<usize>();
    }
    let mut x = vec![true; n];
    for _ in 0..next() { x[next::<usize>()] = false; }

    let mut dp = vec![false; n];
    dp[m] = true;
    for i in 0..k {
        let mut p = dp.clone();
        p.rotate_right(g[i]);
        dp.rotate_right(y[i]);
        for j in 0..n {
            dp[j] |= p[j];
            dp[j] &= x[j];
        }
    }

    println!("{}", if dp[0] { "utopia" } else { "dystopia" });
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