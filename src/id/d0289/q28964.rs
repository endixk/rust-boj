// BOJ 28964 [Library]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut v = vec![];
    for _ in 0..n {
        let (s, f, c) = (next::<usize>(), next::<usize>(), next::<usize>());
        v.push((f, s+c));
    }
    v.sort_unstable();

    let mut t = 0;
    for (f, x) in v {
        if t < x { t = x; }
        if t > f { println!("NO"); return; }
        t += 1;
    }
    println!("YES");
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