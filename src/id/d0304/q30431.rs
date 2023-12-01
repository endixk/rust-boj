// BOJ 30431 [Reverse]
// Supported by GitHub Copilot

fn a(n: usize) {
    println!("{}", n);
    println!("{}", "A".repeat(n));
}
fn b(s: String) {
    if s.len() == 6 {
        println!("1 0 1\n0 0\n0");
    } else {
        println!("2 0 1\n1 1\n0");
    }
}
fn c(s: String) {
    println!("swi's cake is missing!");
    if s == "swi" {
        println!("0\n0");
    } else {
        println!("1\n{}\n0", s);
    }
}
fn d(n: usize) {
    if n == 0 {
        println!("1 1 1 1\n1");
    } else {
        println!("{} {} {} {}", n, n, n, n);
        let s = (0..n).map(|_| "0").collect::<Vec<_>>().join(" ");
        for _ in 0..n*n*n {
            println!("{}", s);
        }
    }
}
fn e(n: usize) {
    if n == 0 {
        println!("1\n1 0");
    } else {
        println!("{}", n*2);
        for i in 1..=n {
            println!("{} {}\n{} {}", i*2, 1, i*2-1, 1);
        }
    }
}
fn f(s: String) {
    println!("{}", s.len());
    for c in s.chars() {
        if c == 'L' { println!("1 2"); }
        else if c == 'D' { println!("1 4"); }
        else { println!("1 1"); }
    }
}
pub fn main() { read();
    a(next());
    b(next());
    c(next());
    d(next());
    e(next());
    f(next());
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