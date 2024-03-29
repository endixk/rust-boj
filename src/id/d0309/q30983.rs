const MOD: u64 = 1_000_000_009;
#[derive(Clone, Copy)] struct Matrix { n: usize, m: [[u64; 100]; 100], }
impl Matrix {
    fn new(n: usize, m: [[u64; 100]; 100]) -> Self {
        Self { n, m }
    }
    fn id(n: usize) -> Self {
        let mut m = [[0; 100]; 100];
        for i in 0..n {
            m[i][i] = 1;
        }
        Self { n, m }
    }
    fn mul(&mut self, rhs: &Self) {
        let mut m = [[0; 100]; 100];
        for i in 0..self.n {
            let rrow = &self.m[i];
            for k in 0..self.n {
                let val = rrow[k];
                let row = &rhs.m[k];
                for j in 0..self.n {
                    m[i][j] = (m[i][j] + val * row[j]) % MOD;
                }
            }
        }
        self.m = m;
    }
    fn pow(&self, mut b: u64) -> Self {
        let mut m = Self::id(self.n);
        let mut a = *self;
        while b > 0 {
            if b & 1 == 1 {
                m.mul(&a);
            }
            let mut x = a;
            x.mul(&a);
            a = x;
            b >>= 1;
        }
        m
    }
}

pub fn main() { read();
    let (n, m, t) = (next::<usize>(), next::<usize>(), next::<u64>());
    let mut adj = [[0; 100]; 100];
    for i in 0..n { adj[i+n][i] = 1; }
    for _ in 0..m {
        let q = next::<u8>();
        if q == 1 {
            let (a, b) = (next::<usize>()-1, next::<usize>()-1);
            adj[a][b] += 1; adj[b][a] += 1;
        } else {
            let (a, b, c) = (next::<usize>()-1, next::<usize>()-1, next::<usize>()-1);
            adj[a][b+n] += 1; adj[a][c+n] += 1;
            adj[b][a+n] += 1; adj[b][c+n] += 1;
            adj[c][a+n] += 1; adj[c][b+n] += 1;
        }
    }

    let mut m = Matrix::new(n*2, adj);
    m = m.pow(t);
    for i in 0..n {
        println!("{}", m.m[i][0]);
    }
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