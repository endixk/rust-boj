// BOJ 30626 [Blaze]
// Supported by GitHub Copilot

pub fn main() { read();
    let mut p = vec![false; 360];
    for _ in 0..next() {
        let (a, b) = (next::<usize>(), next::<usize>());
        let (i, j) = ((a + 180 - b) % 360, (a + 180 + b) % 360);
        if i < j { for k in i..=j { p[k] = true; } }
        else {
            for k in 0..=j { p[k] = true; }
            for k in i..360 { p[k] = true; }
        }
    }

    println!("{}", p.iter().filter(|&&b| b).count());
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