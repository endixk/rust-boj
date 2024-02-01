// BOJ 6575 [Bee Maja]
pub fn main() { read();
    let mut v = vec![(0, 0)];
    let (mut i, mut j, mut k) = (0, 0, 1);
    while v.len() < 100000 {
        j += 1; v.push((i, j));
        for _ in 1..k { i -= 1; j += 1; v.push((i, j)); }
        for _ in 0..k { i -= 1; v.push((i, j)); }
        for _ in 0..k { j -= 1; v.push((i, j)); }
        for _ in 0..k { i += 1; j -= 1; v.push((i, j)); }
        for _ in 0..k { i += 1; v.push((i, j)); }
        for _ in 0..k { j += 1; v.push((i, j)); }
        k += 1;
    }

    while peek() {
        let (x, y) = v[next::<usize>()-1];
        println!("{} {}", x, y);
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<std::iter::Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }