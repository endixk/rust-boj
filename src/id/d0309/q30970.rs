// BOJ 30970 [At the Crossroads]

pub fn main() { read();
    let (mut q1q, mut q1p, mut q2q, mut q2p) = (0, 0, 0, 0);
    let (mut p1q, mut p1p, mut p2q, mut p2p) = (0, 99999, 0, 99999);
    for _ in 0..next() {
        let (q, p) = (next(), next());
        if q > q1q || q == q1q && p < q1p {
            q2q = q1q; q2p = q1p;
            q1q = q; q1p = p;
        } else if q > q2q || q == q2q && p < q2p {
            q2q = q; q2p = p;
        }
        if p < p1p || p == p1p && q > p1q {
            p2q = p1q; p2p = p1p;
            p1q = q; p1p = p;
        } else if p < p2p || p == p2p && q > p2q {
            p2q = q; p2p = p;
        }
    }

    println!("{} {} {} {}", q1q, q1p, q2q, q2p);
    println!("{} {} {} {}", p1q, p1p, p2q, p2p);
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