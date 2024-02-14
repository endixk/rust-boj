// BOJ 6603 [Lotto]
fn go(v: &mut Vec<u8>, a: &[u8], n: usize, i: usize, k: usize) {
    if i == 6 {
        println!("{}", v.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" "));
        return;
    }
    for x in k..n {
        v.push(a[x]);
        go(v, a, n, i+1, x+1);
        v.pop();
    }
}
pub fn main() { read();
    loop {
        let n = next::<usize>();
        if n == 0 { break; }
        let a = (0..n).map(|_| next::<u8>()).collect::<Vec<_>>();
        go(&mut vec![], &a, n, 0, 0);
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
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}