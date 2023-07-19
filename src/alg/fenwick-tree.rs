struct FenwickTree {
    n: usize,
    ft: Vec<i64>,
}
impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            ft: vec![0; n+1],
        }
    }
    fn sum(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.ft[i];
            i -= i & (!i + 1);
        }
        sum
    }
    fn add(&mut self, mut i: usize, v: i64) {
        while i <= self.n {
            self.ft[i] += v;
            i += i & (!i + 1);
        }
    }

    fn point_update(&mut self, i: usize, x: i64) {
        self.add(i, x);
    }
    fn range_query(&self, l: usize, r: usize) -> i64 {
        self.sum(r) - self.sum(l-1)
    }
    fn range_update(&mut self, l: usize, r: usize, v: i64) {
        self.add(l, v);
        self.add(r+1, -v);
    }
    fn point_query(&self, i: usize) -> i64 {
        self.sum(i)
    }
}