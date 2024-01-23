// BOJ 27311 [Chino's Latte Art (Easy)]
const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];
fn fill(a: &Vec<Vec<u8>>, c: &mut Vec<Vec<bool>>, p: &mut (usize, usize, usize, usize),
        i: usize, j: usize, n: usize, m: usize) {
    c[i][j] = true;
    *p = (p.0.min(i), p.1.min(j), p.2.max(i), p.3.max(j));
    for (&x, &y) in DX.iter().zip(DY.iter()) {
        let (i, j) = (i as i32 + x, j as i32 + y);
        if i < 0 || i >= n as i32 || j < 0 || j >= m as i32 { continue; }
        let (i, j) = (i as usize, j as usize);
        if a[i][j] == b'#' && !c[i][j] { fill(a, c, p, i, j, n, m); }
    }
}
pub fn main() { read();
    't: for _ in 0..next() {
        let (n, m) = (next::<usize>(), next::<usize>());
        let a = (0..n).map(|_| next::<String>().into_bytes()).collect::<Vec<_>>();
        let mut c = vec![vec![false; m]; n];
        let mut p = (n, m, 0, 0);
        'x: for i in 0..n {
            for j in 0..m {
                if a[i][j] == b'#' {
                    fill(&a, &mut c, &mut p, i, j, n, m);
                    break 'x;
                }
            }
        }

        for i in 0..n { for j in 0..m {
            if a[i][j] == b'#' && !c[i][j] { println!("0"); continue 't; }
        }}
        if p.0 == n { println!("0"); continue; }
        if p.2 - p.0 != p.3 - p.1 { println!("0"); continue; }
        let k = if a[p.0][p.1] == b'.' { 1 } else { 0 }
            + if a[p.0][p.3] == b'.' { 1 } else { 0 }
            + if a[p.2][p.1] == b'.' { 1 } else { 0 }
            + if a[p.2][p.3] == b'.' { 1 } else { 0 };
        if k != 1 { println!("0"); continue; }

        let mut q = (n, m, 0, 0);
        for i in p.0..=p.2 { for j in p.1..=p.3 {
            if a[i][j] == b'.' {
                q = (q.0.min(i), q.1.min(j), q.2.max(i), q.3.max(j));
            }
        }}
        if q.2 - q.0 != q.3 - q.1 { println!("0"); continue; }
        for i in q.0..=q.2 { for j in q.1..=q.3 {
            if a[i][j] == b'#' { println!("0"); continue 't; }
        }}

        println!("1");
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