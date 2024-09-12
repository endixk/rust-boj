// BOJ 17973 [Quadrilaterals]
// Rotating line sweep; Points must be sorted by x, then -y
#[derive(Clone, Copy, Eq, PartialEq)] struct Point { i: usize, x: i64, y: i64 }
impl Point { fn new(i: usize, x: i64, y: i64) -> Self { Self { i, x, y } } }
#[derive(Clone, Copy, Eq, PartialEq)] struct Segment { i: usize, s: Point, e: Point, ppd: i8 }
impl Segment {
    fn new(i: usize, s: Point, e: Point) -> Self { // s.x < e.x, s.y > e.y
        if s.x == e.x {
            if s.y < e.y { Self { i, e, s, ppd: -1 } } else { Self { i, s, e, ppd: -1 } }
        } else {
            if s.x > e.x { Self { i, e, s, ppd: 0 } } else { Self { i, s, e, ppd: 0 } }
        }
    }
}
impl PartialOrd for Segment {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> { // increasing order of slope
        if self.ppd != rhs.ppd { return Some(self.ppd.cmp(&rhs.ppd)); }
        else if self.ppd == -1 { return Some(self.i.cmp(&rhs.i)); }
        let (dx1, dy1) = (self.e.x - self.s.x, self.e.y - self.s.y);
        let (dx2, dy2) = (rhs.e.x - rhs.s.x, rhs.e.y - rhs.s.y);
        Some((dy1 * dx2).cmp(&(dy2 * dx1))
            .then_with(|| self.i.cmp(&rhs.i)))
    }
}
impl Ord for Segment { fn cmp(&self, rhs: &Self) -> std::cmp::Ordering { self.partial_cmp(rhs).unwrap() } }
fn init(pv: &mut Vec<Point>, n: usize) -> (Vec<(usize, usize)>, Vec<usize>) {
    pv.sort_unstable_by(|a, b| a.x.cmp(&b.x).then_with(|| b.y.cmp(&a.y)));
    let mut map = vec![0; n];
    for i in 0..n { map[pv[i].i] = i; }
    let mut sv = vec![];
    for i in 0..n-1 { for j in i+1..n {
        sv.push((pv[i].i, pv[j].i));
    }}
    sv.sort_unstable_by_key(|&(a, b)| Segment::new(map[a] * n + map[b], pv[map[a]], pv[map[b]]));
    (sv, map)
}
fn sweep(pv: &mut Vec<Point>, map: &mut Vec<usize>, s: usize, e: usize) -> usize {
    // swap seg.s and seg.e
    let (si, ei) = (map[s], map[e]);
    map[s] = ei; map[e] = si;
    pv.swap(si, ei);
    si
}
fn area(p1: &Point, p2: &Point, p3: &Point) -> i64 {
    let (x1, y1) = (p1.x, p1.y);
    let (x2, y2) = (p2.x, p2.y);
    let (x3, y3) = (p3.x, p3.y);
    (x1 * y2 + x2 * y3 + x3 * y1 - x1 * y3 - x2 * y1 - x3 * y2).abs()
}
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
fn update(amin: &mut i64, cnt: &mut usize, px: &Point, ps: &Point, pe: &Point, py: &Point) {
    let a = area(ps, pe, px) + area(pe, ps, py);
    if a < *amin { *amin = a; *cnt = 1; }
    else if a == *amin { *cnt += 1; }
    if a == *amin && ccw(px, ps, py) == ccw(px, pe, py) { *cnt += 1; }
}
pub fn main() { read();
    let n = next::<usize>();
    let mut pv = (0..n).map(|i| Point::new(i, next(), next())).collect::<Vec<_>>();
    let (sv, mut map) = init(&mut pv, n);

    let (mut amin, mut cnt, mut ans) = (i64::MAX, 0, 0);
    for (s, e) in sv {
        let si = sweep(&mut pv, &mut map, s, e);
        let ei = si + 1;
        if si == 0 || ei == n-1 { continue; }
        ans += si * (n - ei - 1);

        update(&mut amin, &mut cnt, &pv[si-1], &pv[si], &pv[ei], &pv[ei+1]);
        if si > 1 { update(&mut amin, &mut cnt, &pv[si-2], &pv[si], &pv[ei], &pv[ei+1]); }
        if ei + 2 < n { update(&mut amin, &mut cnt, &pv[si-1], &pv[si], &pv[ei], &pv[ei+2]); }
        if si > 1 && ei + 2 < n { update(&mut amin, &mut cnt, &pv[si-2], &pv[si], &pv[ei], &pv[ei+2]); }
    }
    println!("{}", ans + cnt);
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