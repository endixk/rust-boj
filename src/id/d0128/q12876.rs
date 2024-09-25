// BOJ 12876 [Half Plane Territories 2]
#[derive(Clone, Copy)] struct Segment { a: i32, b: i32 }
impl Segment {
    fn new(a: i32, b: i32) -> Self { Self { a, b } }
    fn get(&self, x: i32) -> i64 { self.a as i64 * x as i64 + self.b as i64 }
}
struct Node { s: i32, e: i32, l: u32, r: u32, seg: Segment }
impl Node { fn new(s: i32, e: i32, l: u32, r: u32, seg: Segment) -> Self { Self { s, e, l, r, seg } } }
struct LiChaoTree { root: Vec<u32>, tree: Vec<Node>, size: u32 }
impl LiChaoTree {
    fn new(s: i32, e: i32) -> Self {
        Self { root: vec![0], tree: vec![Node::new(s, e, 0, 0, Segment::new(0, i32::MIN))], size: 1 }
    }
    fn push(&mut self, s: i32, e: i32, l: u32, r: u32, seg: Segment) -> u32 {
        self.tree.push(Node::new(s, e, l, r, seg));
        self.size += 1; self.size - 1
    }
    fn update(&mut self, i: u32, seg: Segment) {
        let i = i as usize;
        let (s, e) = (self.tree[i].s, self.tree[i].e);
        let (l, r) = (self.tree[i].l as usize, self.tree[i].r as usize);
        let ori = self.tree[i].seg;
        if ori.b == i32::MIN { self.tree[i].seg = seg; return; }
        let lc = ori.get(s) < seg.get(s);
        let rc = ori.get(e) < seg.get(e);
        if lc ^ rc {
            let m = (s + e) / 2;
            let mc = ori.get(m) < seg.get(m);
            let alt = if mc { self.tree[i].seg = seg; ori } else { seg };
            if lc ^ mc {
                self.tree[i].l = self.push(s, m, self.tree[l].l, self.tree[l].r, self.tree[l].seg);
                self.update(self.tree[i].l, alt);
            } else {
                self.tree[i].r = self.push(m + 1, e, self.tree[r].l, self.tree[r].r, self.tree[r].seg);
                self.update(self.tree[i].r, alt);
            }
        } else if lc { self.tree[i].seg = seg; }
    }
    fn insert(&mut self, seg: Segment) {
        let p = *self.root.last().unwrap() as usize;
        let r = self.push(self.tree[p].s, self.tree[p].e, self.tree[p].l, self.tree[p].r, self.tree[p].seg);
        self.root.push(r);
        self.update(r, seg);
    }
    fn query(&self, i: u32, x: i32) -> i64 {
        let i = i as usize;
        let node = &self.tree[i];
        let mut ret = node.seg.get(x);
        if x <= (node.s + node.e) / 2 {
            if node.l != 0 { ret = ret.max(self.query(node.l, x)) }
        } else if node.r != 0 { ret = ret.max(self.query(node.r, x)) }
        ret
    }
}

struct Qtree { n: usize, tree: Vec<Vec<(i32, i32)>> }
impl Qtree {
    fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        Self { n, tree: vec![vec![]; n << 1] }
    }
    fn insert(&mut self, i: usize, s: u32, e: u32, l: u32, r: u32, a: i32, b: i32) {
        if r < s || e < l { return; }
        if l <= s && e <= r { self.tree[i].push((a, b)); return; }
        let m = (s + e) >> 1;
        self.insert(i << 1, s, m, l, r, a, b);
        self.insert(i << 1 | 1, m + 1, e, l, r, a, b);
    }
    fn query(&self, i: usize, s: u32, e: u32, lct: &mut LiChaoTree, qry: &[i32]) {
        for &(a, b) in &self.tree[i] {
            lct.insert(Segment::new(a, b));
        }
        if s == e {
            let i = i - self.n;
            let r = *lct.root.last().unwrap();
            if i+1 < qry.len() {
                if r == 0 { println!("EMPTY"); }
                else { println!("{}", lct.query(r, qry[i+1])); }
            }
        } else {
            let m = (s + e) >> 1;
            self.query(i << 1, s, m, lct, qry);
            self.query(i << 1 | 1, m + 1, e, lct, qry);
        }
        if !self.tree[i].is_empty() {
            for _ in 1..self.tree[i].len() { lct.root.pop(); }
            let r = lct.root.pop().unwrap();
            lct.tree.truncate(r as usize);
            lct.size = r;
        }
    }
}

pub fn main() {
    let p = &mut input(6161616);
    let mut qry = vec![0];
    let mut segs = vec![];
    let mut map = std::collections::HashMap::new();
    for i in 0..i32(p) {
        match i32(p) {
            3 => qry.push(i32(p)),
            1 => {
                let (a, b) = (i32(p), i32(p));
                segs.push((a, b, qry.len(), usize::MAX));
                map.insert(i+1, segs.len() as i32 - 1);
            },
            _ => {
                let x = i32(p);
                segs[map[&x] as usize].3 = qry.len() - 1;
            }
        }
    }
    for i in 0..segs.len() { if segs[i].3 == usize::MAX { segs[i].3 = qry.len(); } }

    let mut qtree = Qtree::new(qry.len() + 1);
    for &(a, b, s, e) in &segs {
        if s > e { continue; }
        qtree.insert(1, 1, qtree.n as u32, s as u32, e as u32, a, b);
    }
    qtree.query(1, 1, qtree.n as u32, &mut LiChaoTree::new(-(1<<30), 1<<30), &qry);
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn i32(p: &mut *const u8) -> i32 { unsafe {
    let mut n = 0;
    let neg = if **p == b'-' { *p = p.offset(1); true } else { false };
    while **p & 16 != 0 { n = n * 10 + (**p as i32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    if neg { -n } else { n }
}}