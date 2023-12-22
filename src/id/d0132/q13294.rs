// BOJ 13294 [Reverse Factorial]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<String>();
    let n = n.as_str();
    if n.len() < 7 {
        match n {
            "1" => println!("1"),
            "2" => println!("2"),
            "6" => println!("3"),
            "24" => println!("4"),
            "120" => println!("5"),
            "720" => println!("6"),
            "5040" => println!("7"),
            "40320" => println!("8"),
            "362880" => println!("9"),
            _ => unreachable!()
        }
        return;
    }

    let mut i = 10;
    let mut x = 3628800f64.log10();
    loop {
        if n.len() == x as usize + 1 { println!("{}", i); return; }
        i += 1;
        x += (i as f64).log10();
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