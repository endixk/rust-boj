// BOJ 9345 [DVDs]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}
fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

// Max segment tree
struct MaxSegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> MaxSegTree<T> where
    T: Ord + Default + Copy {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![T::default(); m<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1].max(self.v[(i<<1)+1]);
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> T {
        l += self.n; r += self.n;
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
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![T::default(); m<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i += self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1].min(self.v[(i<<1)+1]);
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> T {
        l += self.n; r += self.n;
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let mut arr = (0..=n).collect::<Vec<usize>>();
        let mut xseg = MaxSegTree::new(n+1);
        let mut nseg = MinSegTree::new(n+1);
        for i in 1..=n {
            xseg.update(i, i);
            nseg.update(i, i);
        }

        for _ in 0..k {
            let (q, a, b) = (next::<u8>(&mut it), next::<usize>(&mut it)+1, next::<usize>(&mut it)+1);
            match q {
                0 => {
                    xseg.update(a, arr[b]);
                    xseg.update(b, arr[a]);
                    nseg.update(a, arr[b]);
                    nseg.update(b, arr[a]);
                    arr.swap(a, b);
                }
                _ => {
                    if nseg.query(a, b) == a && xseg.query(a, b) == b {
                        writeln!(so, "YES")?;
                    } else {
                        writeln!(so, "NO")?;
                    }
                }
            }
        }
    }

    Ok(())
}
