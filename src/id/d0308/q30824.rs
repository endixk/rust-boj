// BOJ 30824 [Fibonacci Sum]
// Supported by GitHub Copilot

pub fn main() { read();
    let mut f = vec![1, 1];
    while *f.last().unwrap() < 1e16 as i64 {
        let n = f.len();
        f.push(f[n - 1] + f[n - 2]);
    }

    let mut f2 = vec![];
    for i in 0..f.len() {
        for j in i..f.len() {
            f2.push(f[i] + f[j]);
        }
    }
    f2.sort_unstable();

    let mut f3 = vec![];
    for i in 0..f.len() {
        for j in i..f.len() {
            for k in j..f.len() {
                f3.push(f[i] + f[j] + f[k]);
            }
        }
    }
    f3.sort_unstable();

    for _ in 0..next() {
        let (k, x) = (next::<u8>(), next::<i64>());
        match k {
            1 => println!("{}", if let Ok(_) = f.binary_search(&x) { "YES" } else { "NO" }),
            2 => println!("{}", if let Ok(_) = f2.binary_search(&x) { "YES" } else { "NO" }),
            3 => println!("{}", if let Ok(_) = f3.binary_search(&x) { "YES" } else { "NO" }),
            _ => unreachable!(),
        }
    }
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