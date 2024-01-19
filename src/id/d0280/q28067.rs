// BOJ 28067 [I Love Geometry]
pub fn main() { read();
    let (n, m) = (next::<i32>()+1, next::<i32>()+1);
    let mut set = std::collections::HashSet::new();
    for i in 0..n*m-2 {
        for j in i+1..n*m-1 {
            for k in j+1..n*m {
                let (x1, y1) = (i/m, i%m);
                let (x2, y2) = (j/m, j%m);
                let (x3, y3) = (k/m, k%m);
                let d1 = ((x2 - x1) as f64).hypot((y2 - y1) as f64);
                let d2 = ((x3 - x2) as f64).hypot((y3 - y2) as f64);
                let d3 = ((x3 - x1) as f64).hypot((y3 - y1) as f64);
                if d1 + d2 < d3 + 1e-6 || d2 + d3 < d1 + 1e-6 || d3 + d1 < d2 + 1e-6 {
                    continue;
                }
                let d1 = (x2 - x1).pow(2) + (y2 - y1).pow(2);
                let d2 = (x3 - x2).pow(2) + (y3 - y2).pow(2);
                let d3 = (x3 - x1).pow(2) + (y3 - y1).pow(2);
                let mut v = vec![d1, d2, d3]; v.sort();
                set.insert(v[0]<<20 | v[1]<<10 | v[2]);
            }
        }
    }
    println!("{}", set.len());
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