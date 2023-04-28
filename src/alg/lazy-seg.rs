// Sum Segment Tree with Lazy Propagation
struct SegTree {
    n: usize,
    v: Vec<usize>,
    lazy: Vec<usize>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![0; m<<1], lazy: vec![0; m<<1] }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == 0 { return; }
        self.v[x] += self.lazy[x] * (e - s + 1);
        if s < e {
            self.lazy[x<<1] += self.lazy[x];
            self.lazy[x<<1|1] += self.lazy[x];
        }
        self.lazy[x] = 0;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: usize) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] += v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            self.v[x] = self.v[x<<1] + self.v[x<<1|1];
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> usize {
        self.propagate(x, s, e);
        if r < s || e < l { return 0; }
        if l <= s && e <= r {
            self.v[x]
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r) + self.query(x<<1|1, m+1, e, l, r)
        }
    }
}

// Minimum Segment Tree with Lazy Propagation
struct MinSegTree {
    n: usize,
    v: Vec<usize>,
    lazy: Vec<usize>,
}
const INF: usize = 0x3f3f3f3f;
impl MinSegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![INF; m<<1], lazy: vec![INF; m<<1] }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == INF { return; }
        self.v[x] = self.v[x].min(self.lazy[x]);
        if s < e {
            self.lazy[x<<1] = self.lazy[x].min(self.lazy[x<<1]);
            self.lazy[x<<1|1] = self.lazy[x].min(self.lazy[x<<1|1]);
        }
        self.lazy[x] = INF;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: usize) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] = v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            self.v[x] = self.v[x<<1].min(self.v[x<<1|1]);
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> usize {
        self.propagate(x, s, e);
        if r < s || e < l { return INF; }
        if l <= s && e <= r {
            self.v[x]
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r).min(self.query(x<<1|1, m+1, e, l, r))
        }
    }
}