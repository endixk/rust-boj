// BOJ 26593 [Best Seller]
// Supported by GitHub Copilot

pub fn main() { read();
    let s = unsafe { IT.as_mut().unwrap().map(|s| s.to_string()).collect::<Vec<_>>() };
    let mut v = vec![];
    for i in (0..s.len()).step_by(3) {
        let (a, b, c) = (s[i].clone(), s[i+1].clone().parse::<i32>().unwrap(), (s[i+2].clone().parse::<f64>().unwrap() * 100.0 + 1e-9) as i32);
        v.push((b*c, b, a));
    }
    v.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)).then(a.2.cmp(&b.2)));
    for (a, b, c) in v.iter() {
        println!("${:.2} - {}/{}", *a as f64 / 100.0, c, b);
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