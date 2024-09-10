type T = u32;
#[derive(Default)] struct Node { l: usize, r: usize, c: T }
struct PST { n: usize, tree: Vec<Node>, root: Vec<usize> }
impl PST {
    fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        Self { n, tree: vec![Node::default()], root: vec![0] }
    }

    fn update(&mut self, s: usize, e: usize, i: usize, l: usize, r: usize, c: u32, pos: usize, x: T) {
        if s == e { self.tree[pos].c = c + x; return; }
        let m = s + e >> 1;
        if i <= m {
            let (cl, cr, cc) = (self.tree[l].l, self.tree[l].r, self.tree[l].c);
            let l = self.tree.len(); self.tree.push(Node::default());
            self.update(s, m, i, cl, cr, cc, l, x);
            self.tree[pos].l = l;
            self.tree[pos].r = r;
            self.tree[pos].c = self.tree[l].c + self.tree[r].c;
        } else {
            let (cl, cr, cc) = (self.tree[r].l, self.tree[r].r, self.tree[r].c);
            let r = self.tree.len(); self.tree.push(Node::default());
            self.update(m + 1, e, i, cl, cr, cc, r, x);
            self.tree[pos].l = l;
            self.tree[pos].r = r;
            self.tree[pos].c = self.tree[l].c + self.tree[r].c;
        }
    }

    fn insert(&mut self, i: usize, x: T) {
        let pr = *self.root.last().unwrap();
        let pos = self.tree.len();
        self.root.push(pos);
        self.tree.push(Node::default());
        self.update(0, self.n-1, i, self.tree[pr].l, self.tree[pr].r, self.tree[pr].c, pos, x);
    }

    fn query_cnt(&self, s: usize, e: usize, l: usize, r: usize, lpos: usize, rpos: usize) -> T {
        if r < s || e < l { return 0; }
        if l <= s && e <= r { return self.tree[rpos].c - self.tree[lpos].c; }
        let m = s + e >> 1;
        self.query_cnt(s, m, l, r, self.tree[lpos].l, self.tree[rpos].l) +
            self.query_cnt(m + 1, e, l, r, self.tree[lpos].r, self.tree[rpos].r)
    }
}

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