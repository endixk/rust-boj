// BOJ 28980 [Final Grade]
// Supported by GitHub Copilot

pub fn main() { read();
    let s = next::<String>();
    let s = s.as_bytes();
    let (mut w, mut c) = (26, 0);
    for &x in s {
        let x = b'Z' - x;
        if x < w { w = x; }
        c += x as i32;
    }

    let avg = (c as f64 / s.len() as f64 + 1e-6).round() as u8;
    let avg = if avg > w + 1 { w + 1 } else { avg };
    println!("{}", (b'Z' - avg) as char);
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