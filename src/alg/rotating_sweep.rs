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