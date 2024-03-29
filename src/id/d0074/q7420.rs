// BOJ 7420 [Walls]
// Supported by GitHub Copilot

#[derive(Eq, PartialEq, Clone, Copy, Debug)] struct Point { x: i32, y: i32 }
static mut ORI: Point = Point { x: 0, y: 0 };
impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            ccw(&*std::ptr::addr_of!(ORI), other, self).cmp(&0)
                .then(dist(&*std::ptr::addr_of!(ORI), self).cmp(&dist(&*std::ptr::addr_of!(ORI), other)))
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
fn dist(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x) as i64).pow(2) + ((a.y - b.y) as i64).pow(2)
}

fn graham(mut points: Vec<Point>) -> Vec<Point> {
    // find the lowest point
    let mut loc = 0;
    for i in 1..points.len() {
        if points[i].y < points[loc].y || (points[i].y == points[loc].y && points[i].x < points[loc].x) {
            loc = i;
        }
    }
    points.swap(0, loc);
    unsafe { ORI = points[0].clone(); }

    // sort the points by polar angle
    points.sort_unstable();

    // find the convex hull
    let mut hull = Vec::new();
    hull.push(points[0]);
    hull.push(points[1]);
    for i in 2..points.len() {
        while hull.len() >= 2 {
            let a = hull.pop().unwrap();
            let b = hull.last().unwrap();
            if ccw(b, &a, &points[i]) > 0 {
                hull.push(a);
                break;
            }
        }
        hull.push(points[i]);
    }

    hull
}

pub fn main() { read();
     let (n, l) = (next::<usize>(), next::<f64>());
    let mut points = Vec::new();
    for _ in 0..n {
        points.push(Point { x: next(), y: next() });
    }
    let hull = graham(points);

    let mut ans = 0.0;
    for i in 0..hull.len() {
        let j = (i + 1) % hull.len();
        ans += (dist(&hull[i], &hull[j]) as f64).sqrt();
    }
    println!("{:.0}", ans + 2.0 * std::f64::consts::PI * l);
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