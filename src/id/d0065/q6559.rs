// BOJ 6559 [Bound Found]
pub fn main() { read();
    loop {
        let (n, k) = (next::<usize>(), next::<usize>());
        if n == 0 && k == 0 { break; }
        let a = (0..n).map(|_| next::<i32>()).collect::<Vec<_>>();
        let mut p = vec![(0, 0); n+1];
        for i in 0..n { p[i+1] = (p[i].0 + a[i], i+1); }
        p.sort_unstable();

        for _ in 0..k {
            let x = next::<i32>();
            let mut i = 0;
            let (mut k, mut z, mut l, mut r) = (i32::MAX, 0, 0, 0);
            for j in 1..=n {
                while i < j-1 && p[j].0 - p[i].0 > x { i += 1; }
                if k > (p[j].0 - p[i].0 - x).abs() {
                    k = (p[j].0 - p[i].0 - x).abs();
                    z = p[j].0 - p[i].0;
                    (l, r) = if p[i].1 < p[j].1 { (p[i].1 + 1, p[j].1 )} else { (p[j].1 + 1, p[i].1) };
                }
                if i > 0 && k > (p[j].0 - p[i-1].0 - x).abs() {
                    k = (p[j].0 - p[i-1].0 - x).abs();
                    z = p[j].0 - p[i-1].0;
                    (l, r) = if p[i-1].1 < p[j].1 { (p[i-1].1 + 1, p[j].1 )} else { (p[j].1 + 1, p[i-1].1) };
                }
            }
            println!("{} {} {}", z, l, r);
        }
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