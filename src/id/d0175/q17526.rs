// BOJ 17526 [Star Trek]
#[derive(Clone, Copy)] struct Segment { a: i64, b: i64 }
impl Segment {
    fn new(a: i64, b: i64) -> Self { Self { a, b } }
    fn get(&self, x: i64) -> i64 { self.a * x + self.b }
}
const INF: i64 = 1<<62;
struct Node { s: i64, e: i64, l: usize, r: usize, seg: Segment }
impl Node { fn new(s: i64, e: i64) -> Self { Self { s, e, l: 0, r: 0, seg: Segment::new(0, INF) } } }
struct LiChaoTree { tree: Vec<Node>, size: usize }
impl LiChaoTree {
    fn new(s: i64, e: i64) -> Self {
        Self { tree: vec![Node::new(s, e), Node::new(s, e)], size: 2 }
    }
    fn insert(&mut self, s: i64, e: i64) -> usize {
        self.tree.push(Node::new(s, e));
        self.size += 1; self.size - 1
    }
    fn update(&mut self, i: usize, seg: Segment) {
        let (s, e, ori) = (self.tree[i].s, self.tree[i].e, self.tree[i].seg);
        let lc = ori.get(s) > seg.get(s);
        let rc = ori.get(e) > seg.get(e);
        if lc ^ rc {
            let m = (s + e) / 2;
            let mc = ori.get(m) > seg.get(m);
            let alt = if mc { self.tree[i].seg = seg; ori } else { seg };
            if lc ^ mc {
                if self.tree[i].l == 0 { self.tree[i].l = self.insert(s, m); }
                self.update(self.tree[i].l, alt);
            } else {
                if self.tree[i].r == 0 { self.tree[i].r = self.insert(m + 1, e); }
                self.update(self.tree[i].r, alt);
            }
        } else if lc { self.tree[i].seg = seg; }
    }
    fn query(&self, i: usize, x: i64) -> i64 {
        let node = &self.tree[i];
        let mut ret = node.seg.get(x);
        if x <= (node.s + node.e) / 2 {
            if node.l != 0 { ret = ret.min(self.query(node.l, x)) }
        } else if node.r != 0 { ret = ret.min(self.query(node.r, x)) }
        ret
    }
}
pub fn main() { read();
    let n = next::<usize>() - 1;
    let d = (0..n).map(|_| next::<i64>()).collect::<Vec<_>>();
    let (mut t, mut p, mut c) = (next::<i64>(), next::<i64>(), 0);

    let mut lct = LiChaoTree::new(0, 1<<30);
    lct.update(1, Segment::new(p, t));
    for i in 0..n-1 {
        (t, p, c) = (next::<i64>(), next::<i64>(), c + d[i]);
        let q = lct.query(1, c);
        lct.update(1, Segment::new(p, q + t - p * c));
    }
    println!("{}", lct.query(1, c + d[n-1]));
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