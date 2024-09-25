// BOJ 16783 [Bulldozer]
// Rotating line sweep; Points must be sorted by x, then -y
#[derive(Clone, Copy, Eq, PartialEq)] struct Point { i: usize, x: i64, y: i64 }
impl Point { fn new(i: usize, x: i64, y: i64) -> Self { Self { i, x, y } } }
#[derive(Clone, Copy, Eq, PartialEq)] struct Segment { i: usize, s: Point, e: Point, ppd: i8 }
impl Segment {
    fn new(i: usize, s: Point, e: Point) -> Self { // s.x < e.x, s.y > e.y
        if s.x == e.x {
            if s.y < e.y { Self { i, s: e, e: s, ppd: -1 } } else { Self { i, s, e, ppd: -1 } }
        } else {
            if s.x > e.x { Self { i, s: e, e: s, ppd: 0 } } else { Self { i, s, e, ppd: 0 } }
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
fn compare(s1: &Point, e1: &Point, s2: &Point, e2: &Point) -> bool {
    let (p1, p2) = (s1.x == e1.x, s2.x == e2.x);
    if p1 && p2 { return true; }
    else if p1 ^ p2 { return false; }
    let (dx1, dy1) = (e1.x - s1.x, e1.y - s1.y);
    let (dx2, dy2) = (e2.x - s2.x, e2.y - s2.y);
    dy1 * dx2 == dy2 * dx1
}
// Maximum subarray sum segment tree
type T = i64;
struct SegTree { n: usize, l: Vec<T>, r: Vec<T>, m: Vec<T>, s: Vec<T> }
impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            n: n.next_power_of_two(),
            l: vec![T::default(); n.next_power_of_two()<<1],
            r: vec![T::default(); n.next_power_of_two()<<1],
            m: vec![T::default(); n.next_power_of_two()<<1],
            s: vec![T::default(); n.next_power_of_two()<<1],
        }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i |= self.n;
        self.l[i] = x; self.r[i] = x; self.m[i] = x; self.s[i] = x;
        while i > 1 {
            i >>= 1;
            self.l[i] = self.l[i<<1].max(self.s[i<<1] + self.l[i<<1|1]);
            self.r[i] = self.r[i<<1|1].max(self.s[i<<1|1] + self.r[i<<1]);
            self.m[i] = self.m[i<<1].max(self.m[i<<1|1]).max(self.r[i<<1] + self.l[i<<1|1]);
            self.s[i] = self.s[i<<1] + self.s[i<<1|1];
        }
    }
}
pub fn main() { read();
    let n = next::<usize>();
    let (mut pv, mut c) = (vec![], vec![]);
    for i in 0..n {
        pv.push(Point::new(i, next(), next()));
        c.push(next::<i64>());
    }

    let (sv, mut map) = init(&mut pv, n);
    let mut seg = SegTree::new(n);
    for i in 0..n {
        seg.update(i, c[pv[i].i]);
    }
    let mut ans = 0.max(seg.m[1]);
    let (mut ps, mut pe) = (Point::new(0, pv[0].x, pv[0].y), Point::new(0, pv[0].x, pv[0].y+1));
    for (s, e) in sv {
        let si = sweep(&mut pv, &mut map, s, e); let ei = si + 1;
        if !compare(&pv[si], &pv[ei], &ps, &pe) { ans = ans.max(seg.m[1]); }
        seg.update(si, c[pv[si].i]); seg.update(ei, c[pv[ei].i]);
        (ps, pe) = (pv[si], pv[ei]);
    }
    println!("{}", ans.max(seg.m[1]));
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