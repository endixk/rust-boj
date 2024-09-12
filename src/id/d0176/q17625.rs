// BOJ 17625 [High-Voltage Line]
#[derive(Clone, Copy, Eq, PartialEq)] struct Point { i: usize, x: i32, y: i32 }
impl Point {
    fn new(i: usize, x: i32, y: i32) -> Self { Self { i, x, y } }
    fn dist(&self, rhs: &Self) -> f64 { (self.x as f64 - rhs.x as f64).hypot(self.y as f64 - rhs.y as f64) }
    fn dsq(&self, rhs: &Self) -> i128 { ((self.x - rhs.x) as i128).pow(2) + ((self.y - rhs.y) as i128).pow(2) }
}
fn cmp(i1: usize, s1: &Point, e1: &Point, i2: usize, s2: &Point, e2: &Point) -> std::cmp::Ordering {
    let (s1, e1) = if s1.x == e1.x {
        if s1.y < e1.y { (e1, s1) } else { (s1, e1) }
    } else {
        if s1.x > e1.x { (e1, s1) } else { (s1, e1) }
    };
    let ppd1 = if s1.x == e1.x { -1 } else { 0 };
    let (s2, e2) = if s2.x == e2.x {
        if s2.y < e2.y { (e2, s2) } else { (s2, e2) }
    } else {
        if s2.x > e2.x { (e2, s2) } else { (s2, e2) }
    };
    let ppd2 = if s2.x == e2.x { -1 } else { 0 };
    if ppd1 != ppd2 { return ppd1.cmp(&ppd2); }
    else if ppd1 == -1 { return i1.cmp(&i2); }
    let (dx1, dy1) = (e1.x as i64 - s1.x as i64, e1.y as i64 - s1.y as i64);
    let (dx2, dy2) = (e2.x as i64 - s2.x as i64, e2.y as i64 - s2.y as i64);
    (dy1 * dx2).cmp(&(dy2 * dx1))
        .then_with(|| i1.cmp(&i2))
}
fn init(pv: &mut Vec<Point>, n: usize) -> (Vec<(Point, Point, bool)>, Vec<usize>) {
    pv.sort_unstable_by(|a, b| a.x.cmp(&b.x).then_with(|| b.y.cmp(&a.y)));
    let mut map = vec![0; n];
    for i in 0..n { map[pv[i].i] = i; }
    let mut sv = vec![];
    for i in 0..n-1 { for j in i+1..n {
        sv.push((pv[i], pv[j], false));
        sv.push((pv[i], pv[j], true));
    }}
    sv.sort_unstable_by(|&(a1, b1, t1), &(a2, b2, t2)| {
        let (i, a1, b1) = if t1 {
            (0, Point::new(0, b1.y, a1.x), Point::new(0, a1.y, b1.x))
        } else {
            (map[a1.i] * n + map[b1.i], a1, b1)
        };
        let (j, a2, b2) = if t2 {
            (0, Point::new(0, b2.y, a2.x), Point::new(0, a2.y, b2.x))
        } else {
            (map[a2.i] * n + map[b2.i], a2, b2)
        };
        cmp(i, &a1, &b1, j, &a2, &b2)
    });
    (sv, map)
}
fn sweep(pv: &mut Vec<Point>, map: &mut Vec<usize>, s: usize, e: usize) -> usize {
    // swap seg.s and seg.e
    let (si, ei) = (map[s], map[e]);
    map[s] = ei; map[e] = si;
    pv.swap(si, ei);
    si
}
fn area(p1: &Point, p2: &Point, p3: &Point) -> i128 {
    (p1.x as i128 * p2.y as i128 + p2.x as i128 * p3.y as i128 + p3.x as i128 * p1.y as i128
        - p1.x as i128 * p3.y as i128 - p2.x as i128 * p1.y as i128 - p3.x as i128 * p2.y as i128).abs()
}
pub fn main() { read();
    let n = next::<usize>();
    let mut pv = (0..n).map(|i| Point::new(i, next(), next())).collect::<Vec<_>>();
    let (sv, mut map) = init(&mut pv, n);
    let mut ans = 0.0f64;
    for (s, e, t) in sv {
        if t {
            let (si, ei) = (map[s.i], map[e.i]);
            if si.abs_diff(ei) == 1 {
                ans = ans.max(pv[si].dsq(&pv[ei]) as f64 / 4.0);
            }
            continue;
        }
        let si = sweep(&mut pv, &mut map, s.i, e.i);
        let ei = si + 1;
        let dsq = pv[si].dsq(&pv[ei]);
        if si > 0 { ans = ans.max(area(&pv[si-1], &pv[si], &pv[ei]).pow(2) as f64 / 4.0 / dsq as f64); }
        if ei < n-1 { ans = ans.max(area(&pv[si], &pv[ei], &pv[ei+1]).pow(2) as f64 / 4.0 / dsq as f64); }
    }
    println!("{:.12}", ans.sqrt());
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