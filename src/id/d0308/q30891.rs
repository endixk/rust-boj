// BOJ 30891 [Fried Rice]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, r) = (next::<usize>(), next::<i32>());
    let p = (0..n).map(|_| (next::<i32>(), next::<i32>())).collect::<Vec<_>>();

    let (mut x, mut y, mut k) = (0, 0, 0);
    for i in -100..=100 {
        for j in -100..=100 {
            let mut c = 0;
            for (a, b) in &p {
                if (a - i).pow(2) + (b - j).pow(2) <= r.pow(2) { c += 1; }
            }
            if c > k { k = c; x = i; y = j; }
        }
    }
    println!("{} {}", x, y);
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