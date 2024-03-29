// BOJ 30624 [Identical Sceneries]
// Supported by GitHub Copilot

#[derive(Eq, PartialEq, Clone, Copy, Debug)] struct Point { x: i32, y: i32 }
static mut ORI: Point = Point { x: 0, y: 0 };
impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            ccw(&*std::ptr::addr_of!(ORI), other, self).cmp(&0)
                .then(dsq(&*std::ptr::addr_of!(ORI), self).cmp(&dsq(&*std::ptr::addr_of!(ORI), other)))
        }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
fn dsq(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x) as i64).pow(2) + ((a.y - b.y) as i64).pow(2)
}

pub fn main() { read();
    let (n, m) = (next(), next());
    unsafe { ORI = Point { x: next(), y: next() }; }
    let mut p = (0..n).map(|_| Point { x: next(), y: next() }).collect::<Vec<_>>();
    let q = (0..m).map(|_| next::<i32>()).collect::<Vec<_>>();

    p.sort_unstable();
    let (mut l, mut r) = (-1e10f64, 1e10f64);
    for i in 1..p.len() {
        if p[i-1].y == p[i].y { continue; }
        let k = (p[i-1].x as f64 * p[i].y as f64 - p[i].x as f64 * p[i-1].y as f64) / (p[i].y - p[i-1].y) as f64;
        if p[i-1].y > p[i].y { l = l.max(k); } else { r = r.min(k); }
    }

    println!("{}", q.iter().filter(|&&x| l < (x as f64) && (x as f64) < r).count());
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