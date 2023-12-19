// BOJ 28946 [Queue to the Attraction]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, m, k) = (next::<usize>(), next::<usize>(), next::<usize>());
    let a = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let t = (0..m).map(|_| next::<usize>()).collect::<Vec<_>>();
    if n % 2 == 0 {
        println!("{} {}", a.iter().sum::<usize>(), 0);
    } else {
        let x = t.iter().filter(|&x| x <= &k).count();
        println!("{} {}", a[1..].iter().sum::<usize>(), x);
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