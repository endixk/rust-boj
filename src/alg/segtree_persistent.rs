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