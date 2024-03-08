// BOJ 6544 [Fixed Partition Contest Management]
// TODO: SPJ not implemented, impossible to solve
const INF: usize = 0x3f3f3f3f;
pub fn main() { read();
    for tc in 1.. {
        let (m, n) = (next::<usize>(), next::<usize>());
        if m == 0 && n == 0 { break; }

        let a = (0..m).map(|_| next::<usize>()).collect::<Vec<_>>();
        let mut v = vec![vec![INF; n]; m];
        for j in 0..n {
            for _ in 0..next() {
                let (s, k) = (next::<usize>(), next::<usize>());
                for i in 0..m {
                    if a[i] >= s { v[i][j] = k; }
                }
            }
        }

        let (mut mbt, mut min) = (vec![], INF);
        'x: for bit in 0..m.pow(n as u32) {
            let mut w = vec![0; n];
            let mut b = bit;
            for j in 0..n { w[j] = b % m; b /= m; }

            let mut t = vec![vec![]; m];
            for j in 0..n {
                if v[w[j]][j] == INF { continue 'x; }
                t[w[j]].push((v[w[j]][j], j));
            }

            let mut s = 0;
            for v in t.iter_mut() {
                v.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
                let mut p = 0;
                for &mut (x, _) in v {
                    p += x; s += p;
                }
            }

            if s <= min { min = s; mbt = t.clone(); }
        }

        let mut t = vec![(0, 0, 0); n];
        for (i, v) in mbt.into_iter().enumerate() {
            let mut s = 0;
            for (x, j) in v {
                t[j] = (i+1, s, s+x);
                s += x;
            }
        }

        println!("Case {}", tc);
        println!("Average solution time = {:.2}", min as f64 / n as f64);
        for (i, (x, s, e)) in t.into_iter().enumerate() {
            println!("Problem {} is solved by member {x} from {s} to {e}", i+1);
        }
        println!();
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}