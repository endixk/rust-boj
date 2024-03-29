// Sum segment tree
struct SegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> SegTree<T> where
    T: std::ops::AddAssign + std::ops::Add<Output=T> + Default + Copy {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), v: vec![T::default(); n.next_power_of_two()<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i |= self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[i<<1|1];
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> T {
        l |= self.n; r |= self.n;
        let mut ans = T::default();
        while l <= r {
            if l & 1 == 1 { ans += self.v[l]; l += 1; }
            if r & 1 == 0 { ans += self.v[r]; r -= 1; }
            l >>= 1; r >>= 1;
        }
        ans
    }
}

// Max segment tree
struct MaxSegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> MaxSegTree<T> where
    T: Ord + Default + Copy {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), v: vec![T::default(); n.next_power_of_two()<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i |= self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1].max(self.v[i<<1|1]);
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> T {
        l |= self.n; r |= self.n;
        let mut ans: Option<T> = None;
        while l <= r {
            if l & 1 == 1 {
                if let Some(x) = ans { ans = Some(x.max(self.v[l])); }
                else { ans = Some(self.v[l]); }
                l += 1;
            }
            if r & 1 == 0 {
                if let Some(x) = ans { ans = Some(x.max(self.v[r])); }
                else { ans = Some(self.v[r]); }
                r -= 1;
            }
            l >>= 1; r >>= 1;
        }
        ans.unwrap()
    }
}

// Min segment tree
struct MinSegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> MinSegTree<T> where
    T: Ord + Default + Copy {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), v: vec![T::default(); n.next_power_of_two()<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i |= self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1].min(self.v[i<<1|1]);
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> T {
        l |= self.n; r |= self.n;
        let mut ans: Option<T> = None;
        while l <= r {
            if l & 1 == 1 {
                if let Some(x) = ans { ans = Some(x.min(self.v[l])); }
                else { ans = Some(self.v[l]); }
                l += 1;
            }
            if r & 1 == 0 {
                if let Some(x) = ans { ans = Some(x.min(self.v[r])); }
                else { ans = Some(self.v[r]); }
                r -= 1;
            }
            l >>= 1; r >>= 1;
        }
        ans.unwrap()
    }
}

// Count segment tree
struct CountSegTree {
    n: usize,
    c: Vec<i32>,
    v: Vec<usize>,
}
impl CountSegTree {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), c: vec![0; n.next_power_of_two()<<1], v: vec![0; n.next_power_of_two()<<1] }
    }
    fn update(&mut self, i: usize, s: usize, e: usize, l: usize, r: usize, x: i32) {
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.c[i] += x;
        } else {
            let m = (s + e) >> 1;
            self.update(i<<1, s, m, l, r, x);
            self.update((i<<1)+1, m+1, e, l, r, x);
        }
        if self.c[i] > 0 {
            self.v[i] = e - s + 1;
        } else {
            if s == e { self.v[i] = 0; }
            else { self.v[i] = self.v[i<<1] + self.v[i<<1|1]; }
        }
    }
}

// Merge sort tree
struct MergeSortTree<T> { n: usize, v: Vec<Vec<T>> }
impl<T> MergeSortTree<T> where T: Ord + Default + Copy {
    fn merge(a: &[T], b: &[T]) -> Vec<T> {
        let (mut i, mut j) = (0, 0);
        let (m, n) = (a.len(), b.len());
        let mut c = Vec::with_capacity(m + n);
        while i < m && j < n {
            if a[i] < b[j] { c.push(a[i]); i += 1; }
            else { c.push(b[j]); j += 1; }
        }
        while i < m { c.push(a[i]); i += 1; }
        while j < n { c.push(b[j]); j += 1; }
        c
    }
    fn new(n: usize, a: &[T]) -> Self {
        let mut m = n.next_power_of_two();
        let mut v = vec![vec![]; m<<1];
        for i in 0..n { v[i+m] = vec![a[i]]; }
        for i in (1..m).rev() { v[i] = Self::merge(&v[i<<1], &v[i<<1|1]); }
        Self { n: m, v }
    }
    fn query(&self, mut l: usize, mut r: usize, x: T, y: T) -> usize {
        if l > r { return 0; }
        if x > y { return 0; }
        l |= self.n; r |= self.n;
        let mut ans = 0;
        while l <= r {
            if l & 1 == 1 {
                ans += self.v[l].partition_point(|&z| z <= y) - self.v[l].partition_point(|&z| z < x);
                l += 1;
            }
            if r & 1 == 0 {
                ans += self.v[r].partition_point(|&z| z <= y) - self.v[r].partition_point(|&z| z < x);
                r -= 1;
            }
            l >>= 1; r >>= 1;
        }
        ans
    }
}