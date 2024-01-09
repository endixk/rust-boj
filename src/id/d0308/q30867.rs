// BOJ 30867 [Too Many Assignments]
// Supported by GitHub Copilot

pub fn main() { read();
    let (l, n) = (next::<usize>(), next::<usize>());
    let s = next::<String>().chars().collect::<Vec<_>>();

    let mut k = vec![0; l];
    let mut x = 0;
    for i in 0..l {
        if s[i] == 'w' { x += 1; }
        else if s[i] == 'h' { k[i] = x; }
        else { x = 0; }
    }

    let mut t = s.clone();
    for i in 0..l {
        if k[i] > 0 {
            t[i - if k[i] < n { k[i] } else { n }] = 'h';
            t[i] = 'w';
        }
    }
    println!("{}", t.iter().collect::<String>());
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