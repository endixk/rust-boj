// BOJ 31063 [Candy Cane Feast]
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut h = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    for _ in 0..m {
        let x = next::<usize>();
        let mut k = 0;
        for i in 0..n {
            if h[i] >= k {
                if h[i] >= x { h[i] += x - k; break; }
                let t = h[i]; h[i] += h[i] - k; k = t;
            }
        }
    }
    h.iter().for_each(|x| println!("{}", x));
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