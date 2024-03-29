// BOJ 31417 [The Shortest Height]
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)] struct Point { x: i32, y: i32 }
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { println!("0"); std::process::exit(0); }
}
fn dist(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x) as f64).hypot((a.y - b.y) as f64)
}
fn h(a: &Point, b: &Point, c: &Point) -> f64 {
    let s = (a.x - c.x) as i64 * (b.y - c.y) as i64 - (a.y - c.y) as i64 * (b.x - c.x) as i64;
    s.abs() as f64 / dist(b, c)
}
pub fn main() { read();
    let n = next::<usize>();
    let p = (0..n).map(|_| Point { x: next(), y: next() }).collect::<Vec<_>>();

    let mut ans = f64::MAX;
    for i in 0..n {
        let mut p = p.clone();
        p.swap(0, i);
        let ori = p[0];
        p[1..].sort_unstable_by(|a, b| {
            ccw(&ori, b, a).cmp(&0)
        });
        p.push(p[1]);

        for i in 2..=n {
            let (a, b) = (&p[i-1], &p[i]);
            ans = ans.min(h(a, b, &ori));
            ans = ans.min(h(b, a, &ori));
        }
    }
    println!("{:.7}", ans);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}