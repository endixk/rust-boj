// BOJ 26587 [Reverse]
// Supported by GitHub Copilot

pub fn main() { read();
    let v = unsafe { IT.as_mut().unwrap().map(|s| s.to_string()).collect::<Vec<_>>() };
    for s in v {
        let w = s.split_whitespace().collect::<Vec<_>>();
        let mut c = w.iter().filter(|&&s| "aeiouAEIOU".contains(s.chars().next().unwrap())).collect::<Vec<_>>();

        for x in &w {
            if "aeiouAEIOU".contains(x.chars().next().unwrap()) {
                print!("{} ", c.pop().unwrap());
            } else {
                print!("{} ", x);
            }
        }
        println!();
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<std::str::Split<'static, &'static str>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split("\n"));
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}