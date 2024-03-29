// BOJ 31564 [Hexagon Maze Escape]
const DX: [usize; 6] = [!0, !0, 0, 0, 1, 1];
const DYE: [usize; 6] = [!0, 0, !0, 1, !0, 0];
const DYO: [usize; 6] = [0, 1, !0, 1, 0, 1];
pub fn main() { read();
    let (n, m, k) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut v = vec![vec![false; m]; n];
    for _ in 0..k {
        let (i, j) = (next::<usize>(), next::<usize>());
        v[i][j] = true;
    }

    let mut q = vec![(0usize, 0usize)];
    v[0][0] = true;
    let mut d = 0;
    while !q.is_empty() {
        let mut nq = vec![];
        for (i, j) in q {
            for (&dx, &dy) in DX.iter().zip(if i & 1 == 0 { DYE.iter() } else { DYO.iter() }) {
                let (ni, nj) = (i.wrapping_add(dx), j.wrapping_add(dy));
                if ni == n - 1 && nj == m - 1 {
                    println!("{}", d + 1);
                    return;
                }
                if ni < n && nj < m && !v[ni][nj] {
                    v[ni][nj] = true;
                    nq.push((ni, nj));
                }
            }
        }
        q = nq; d += 1;
    }
    println!("-1");
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