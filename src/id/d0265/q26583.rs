// BOJ 26583 [Scale]
// Supported by GitHub Copilot

pub fn main() {
    while read() {
        let mut v = unsafe { IT.as_mut().unwrap().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>() };
        v.push(1); v.push(1); v.rotate_right(1);
        for i in 2..v.len() {
            print!("{} ", v[i-2] * v[i-1] * v[i]);
        }
        println!();
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
fn read() -> bool { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_line(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
    BUF.len() > 0
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}