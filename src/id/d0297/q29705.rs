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

const MOD: u64 = 1_000_000_007;
#[derive(Clone)]
struct Matrix {
    n: usize,
    m: Vec<Vec<u64>>,
}
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
            for j in 0..self.n {
                for k in 0..self.n {
                    m[i][j] = (m[i][j] + self.m[i][k] * rhs.m[k][j]) % MOD;
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

fn get(mat: &Matrix, n: usize) -> u64 {
    mat.pow(n as u64).m[0][0]
}
fn proc(f: &mut Vec<u64>, k: &[u64]) {
    let mut x = 0;
    for i in 0..9 {
        x = (x + f[i] * k[i]) % MOD;
    }
    for i in (1..9).rev() { f[i] = f[i - 1]; }
    f[0] = x;
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (a, b) = (next::<u64>(&mut it), next::<u64>(&mut it));
    let k = (0..9).map(|_| next::<String>(&mut it).len() as u64).collect::<Vec<_>>();

    let mut mat = vec![vec![0; 9]; 9];
    for i in 0..9 {
        if i == 0 {
            for j in 0..9 {
                mat[i][j] = k[j];
            }
        } else {
            mat[i][i - 1] = 1;
        }
    }
    let mat = Matrix::new(9, mat);

    let mut f = vec![0; 9];
    for i in 0..9 {
        if a as usize > i {
            f[i] = get(&mat, a as usize - i);
        }
    }
    if a < 9 { f[a as usize] = 1; }

    let mut ans = 0;
    for _ in a..=b {
        ans = (ans + f[0]) % MOD;
        proc(&mut f, &k);
    }
    writeln!(so, "{}", ans).unwrap();
}