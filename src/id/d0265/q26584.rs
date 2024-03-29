// BOJ 26584 [Playtime]
// Supported by GitHub Copilot

pub fn main() { read();
    for _ in 0..next() {
        let s = next::<String>();
        let v = s.split(",").collect::<Vec<_>>();
        print!("{} - ", v[0]);
        let mut x = v[1].parse::<i64>().unwrap();
        if x > 525600 { print!("{} year(s) ", x / 525600); x %= 525600; }
        if x > 1440 { print!("{} day(s) ", x / 1440); x %= 1440; }
        if x > 60 { print!("{} hour(s) ", x / 60); x %= 60; }
        println!("{} minute(s)", x);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split("\n"));
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}