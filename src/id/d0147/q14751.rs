// BOJ 14751 [Leftmost Segment]
fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b) } }
#[derive(Copy, Clone, Debug)] struct Frac { n: i64, d: i64 }
impl Frac {
    fn new(n: i64, d: i64) -> Self {
        let g = gcd(n.abs(), d.abs());
        Self { n: n / g, d: d / g }
    }
    fn default() -> Self { Self { n: 0, d: 1 } }
}
impl Eq for Frac {}
impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool { self.n * other.d == self.d * other.n }
}
impl Ord for Frac {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (self.n * other.d).cmp(&(self.d * other.n)) }
}
impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}

#[derive(Copy, Clone, Debug)]
struct Segment { a: i64, b: i64, s: Frac, i: usize }
fn sect(p: &Segment, q: &Segment) -> Frac {
    Frac::new(q.b - p.b, p.a - q.a)
}
fn find(sv: &[Frac], x: &Frac) -> usize {
    sv.binary_search_by(|&y| y.partial_cmp(x).unwrap()).unwrap_or_else(|i| i) - 1
}

pub fn main() { read();
    let (p, q) = (next::<i64>(), next::<i64>());
    let n = next::<usize>();
    let mut v = (0..n).map(|i| {
        let (a, b) = (next::<i64>(), next::<i64>());
        (b-a, a, i+1)
    }).collect::<Vec<_>>();
    v.sort_unstable_by(|x, y| y.0.cmp(&x.0).then(x.1.cmp(&y.1)));

    // remove segments with same slope
    let mut w = vec![v[0]];
    for i in 1..n {
        if v[i].0 != w.last().unwrap().0 { w.push(v[i]); }
    }
    let n = w.len();

    // find initial segment
    let (mut k, mut x) = (0, 999999);
    for i in 0..n { if w[i].1 <= x { k = i; x = w[i].1; } }

    // collect leftmost hull
    let mut cht = vec![];
    cht.push(Segment { a: w[k].0, b: w[k].1, s: Frac::default(), i: w[k].2 });
    for i in (k+1)..n {
        let mut ns = Segment { a: w[i].0, b: w[i].1, s: Frac::default(), i: w[i].2 };
        while let Some(&p) = cht.last() {
            ns.s = sect(&p, &ns);
            if p.s < ns.s { break; }
            cht.pop();
        }
        cht.push(ns);
    }
    let sv = cht.iter().map(|s| s.s).collect::<Vec<_>>();

    // process queries
    for _ in 0..next() {
        let x = Frac::new(p * 1000 - (next::<f64>() * 1000.0).round() as i64, 1000 * (p - q));
        println!("{}", cht[find(&sv, &x)].i);
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