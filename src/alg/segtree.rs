// Sum segment tree
type T = u64;
struct SegTree { n: usize, v: Vec<T>, }
impl SegTree {
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
type T = u64;
struct MaxSegTree { n: usize, v: Vec<T>, }
impl MaxSegTree {
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
type T = u64;
struct MinSegTree { n: usize, v: Vec<T>, }
impl MinSegTree {
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
type T = i32;
struct CountSegTree {
    n: usize,
    c: Vec<T>,
    v: Vec<usize>,
}
impl CountSegTree {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), c: vec![0; n.next_power_of_two()<<1], v: vec![0; n.next_power_of_two()<<1] }
    }
    fn update(&mut self, i: usize, s: usize, e: usize, l: usize, r: usize, x: T) {
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
type T = i64;
struct MergeSortTree { n: usize, v: Vec<Vec<T>> }
impl MergeSortTree {
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