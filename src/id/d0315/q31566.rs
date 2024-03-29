// BOJ 31566 [Mighty Fine Morning]
const INF: i32 = 0x3f3f3f3f;
pub fn main() { read();
    let (n, m, q) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut d = vec![vec![INF; n]; n];
    for i in 0..n { d[i][i] = 0; }
    for _ in 0..m {
        let (b, t, c) = (next::<usize>() - 1, next::<usize>() - 1, next::<i32>());
        d[b][t] = c;
    }

    let mut r = vec![];
    for i in 0..n {
        let mut mat = d.clone();
        for j in 0..n {
            mat[i][j] = INF; mat[j][i] = INF;
        }
        for k in 0..n { for i in 0..n { for j in 0..n {
            mat[i][j] = mat[i][j].min(mat[i][k] + mat[k][j]);
        }}}
        r.push(mat);
    }

    for _ in 0..q {
        let (s, k, e) = (next::<usize>() - 1, next::<usize>() - 1, next::<usize>() - 1);
        println!("{}", if r[k][s][e] == INF { "No".to_string() } else { r[k][s][e].to_string() });
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