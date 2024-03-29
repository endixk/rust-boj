// BOJ 31504 [Sorted Fractal Sequence]
#[derive(Clone)] struct Matrix { n: usize, m: Vec<Vec<i64>>, }
impl Matrix {
    fn new(n: usize, m: Vec<Vec<i64>>) -> Self {
        Self { n, m }
    }
    fn id(n: usize) -> Self {
        let mut m = vec![vec![0; n]; n];
        for i in 0..n {
            m[i][i] = 1;
        }
        Self { n, m }
    }
    fn mul(&mut self, rhs: &Self, div: i64) {
        let mut m = vec![vec![0; self.n]; self.n];
        for i in 0..self.n {
            let rrow = &self.m[i];
            for k in 0..self.n {
                let val = rrow[k];
                let row = &rhs.m[k];
                for j in 0..self.n {
                    m[i][j] = (m[i][j] + val * row[j]) % div;
                }
            }
        }
        self.m = m;
    }
    fn pow(&self, mut b: usize, div: i64) -> Self {
        let mut m = Self::id(self.n);
        let mut a = self.clone();
        while b > 0 {
            if b & 1 == 1 {
                m.mul(&a, div);
            }
            let mut x = a.clone();
            x.mul(&a, div);
            a = x;
            b >>= 1;
        }
        m
    }
}
fn get(x: usize, div: usize) -> usize {
    if x == 0 { return 0; }
    let mat = Matrix::new(2, vec![vec![3, -1], vec![1, 0]]);
    let mat = mat.pow(x-1, div as i64);
    (mat.m[0][0] + div as i64) as usize % div
}
pub fn main() { read();
    let (n, m, k) = (next::<usize>(), next::<usize>(), next::<usize>());
    if k == 0 { println!("{}", get(n, m)); return; }

    let mut r = vec![];
    for _ in 0..k {
        let (i, v) = (next::<usize>(), next::<usize>());
        r.push(if i < v { (v, i, v) } else { (v, v, i) });
    }
    r.sort_unstable();

    let mut t = Vec::<(usize, usize, usize)>::new();
    for (x, s, e) in r {
        if !t.is_empty() && t.last().unwrap().0 == x {
            t.last_mut().unwrap().2 = e;
        } else { t.push((x, s, e)); }
    }

    let mut k = 0;
    for &(_, s, e) in t.iter() {
        if s <= k { println!("0"); return; }
        k = e;
    }

    let mut ans = (get(t[0].1, m) - get(t[0].1 - 1, m) + 2 * m) % m;
    for i in 1..t.len() {
        ans = ans * get(t[i].1 - t[i-1].2, m) % m;
    }
    ans = ans * (get(n - t.last().unwrap().2 + 1, m) - get(n - t.last().unwrap().2, m) + 2 * m) % m;
    println!("{}", ans);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}