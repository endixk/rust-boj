// BOJ 27318 [World's Sweetest Dessert]
const MOD: u64 = 1_000_000_007;
#[derive(Clone)] struct Matrix { n: usize, m: Vec<Vec<u64>>, }
impl Matrix {
    fn new(n: usize, m: Vec<Vec<u64>>) -> Self {
        Self { n, m }
    }
    fn id(n: usize) -> Self {
        let mut m = vec![vec![0; n]; n];
        for i in 0..n {
            m[i][i] = 1;
        }
        Self { n, m }
    }
    fn mul(&mut self, rhs: &Self) {
        let mut m = vec![vec![0; self.n]; self.n];
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
        let mut a = self.clone();
        while b > 0 {
            if b & 1 == 1 {
                m.mul(&a);
            }
            let mut x = a.clone();
            x.mul(&a);
            a = x;
            b >>= 1;
        }
        m
    }
}
pub fn main() { read();
    let (n, m) = (next::<u64>(), next::<u64>());
    let mut mat = Matrix::new(2, vec![vec![12*n-16, 0], vec![24*n-48, 4*n-4]]);
    mat = mat.pow(m);
    println!("{} {}", (mat.m[0][0] + 6*mat.m[0][1]) % MOD, (mat.m[1][0] + 6*mat.m[1][1]) % MOD);
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