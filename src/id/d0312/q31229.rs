fn sieve(n: usize) -> Vec<usize> {
    let mut p = vec![true; n+1];
    p[0] = false; p[1] = false;
    for i in 2..=n {
        if p[i] {
            for j in (2*i..=n).step_by(i) { p[j] = false; }
        }
    }
    p.iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i).collect()
}
pub fn main() { read();
    let n = next::<usize>();
    let p = sieve(1000000);
    p[..n].iter().for_each(|&x| print!("{} ", x));
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