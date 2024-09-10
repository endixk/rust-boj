// BOJ 31397 [Bon Split (Hard)]
#[derive(Clone, Copy)] struct Point { x: f64, y: f64 }
impl Point {
    fn new(x: f64, y: f64) -> Self { Self { x, y } }
    fn dist(&self, rhs: &Self) -> f64 { (self.x - rhs.x).hypot(self.y - rhs.y) }
}
fn pd(poly: &[Point], p: usize) -> f64 { poly[p].dist(&poly[(p+1)%poly.len()]) }
fn circ(poly: &[Point]) -> f64 { (0..poly.len()).fold(0.0, |acc, i| { acc + pd(poly, i) }) }
fn area(poly: &[Point]) -> f64 {
    (0..poly.len()).fold(0.0, |acc, i| {
        acc + poly[i].x * poly[(i+1)%poly.len()].y - poly[i].y * poly[(i+1)%poly.len()].x
    }).abs() / 2.0
}
fn pc(poly: &[Point], c: f64, p: usize, x: f64) -> (usize, f64) {
    let mut c = c;
    if (1.0 - x) * pd(poly, p) > c { return (p, x + c / pd(poly, p)); }
    c -= (1.0 - x) * pd(poly, p);
    for i in p+1.. {
        let i = i % poly.len();
        if c < pd(poly, i) { return (i, c / pd(poly, i)); }
        c -= pd(poly, i);
    }
    unreachable!()
}
fn diff(poly: &[Point], p1: usize, x1: f64, c: f64, a: f64) -> f64 {
    let (p2, x2) = pc(poly, c, p1, x1);
    let po1 = Point::new(
        poly[p1].x + x1 * (poly[(p1+1)%poly.len()].x - poly[p1].x),
        poly[p1].y + x1 * (poly[(p1+1)%poly.len()].y - poly[p1].y)
    );
    let po2 = Point::new(
        poly[p2].x + x2 * (poly[(p2+1)%poly.len()].x - poly[p2].x),
        poly[p2].y + x2 * (poly[(p2+1)%poly.len()].y - poly[p2].y)
    );

    let mut poly1 = vec![po1, poly[p1]];
    let mut i = p1;
    while i != p2 { i = (i+1) % poly.len(); poly1.push(poly[i]); }
    poly1.push(po2);

    area(&poly1) * 2.0 - a
}
pub fn main() { read();
    let n = next::<usize>();
    let poly = (0..n).map(|_| Point::new(next(), next())).collect::<Vec<_>>();
    let c = circ(&poly);
    let a = area(&poly);

    let (mut pl, mut xl) = (0, 0.0);
    let mut f = c / 4.0;
    for _ in 0..50 {
        let dl = diff(&poly, pl, xl, c / 2.0, a);
        let (pm, xm) = pc(&poly, f, pl, xl);
        let df = diff(&poly, pm, xm, c / 2.0, a);
        if !((dl > 0.0) ^ (df > 0.0)) { (pl, xl) = (pm, xm); }
        f /= 2.0;
    }

    println!("YES");
    println!("{} {:.10}", pl+1, xl);
    (pl, xl) = pc(&poly, c / 2.0, pl, xl);
    println!("{} {:.10}", pl+1, xl);
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}