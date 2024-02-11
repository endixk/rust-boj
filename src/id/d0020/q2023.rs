// BOJ 2023 [Marvelous Primes]
fn isp(n: usize) -> bool {
    if n < 2 { return false }
    for i in 2..n {
        if i*i > n { break }
        if n % i == 0 { return false }
    }
    true
}
fn go(n: usize, i: usize, s: &mut Vec<u8>, x: usize) {
    if i == n { println!("{}", String::from_utf8(s.clone()).unwrap()); return; }
    for j in [1, 3, 7, 9] {
        let y = x*10 + j;
        if isp(y) {
            s.push(j as u8 + b'0');
            go(n, i+1, s, y);
            s.pop();
        }
    }
}
pub fn main() { read();
    let n = next::<usize>();
    go(n, 1, &mut vec![b'2'], 2);
    go(n, 1, &mut vec![b'3'], 3);
    go(n, 1, &mut vec![b'5'], 5);
    go(n, 1, &mut vec![b'7'], 7);
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