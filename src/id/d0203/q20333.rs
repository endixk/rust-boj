// BOJ 20333 [Elevator Pitch]
const DX: [usize; 4] = [0, 0, 1, !0];
const DY: [usize; 4] = [1, !0, 0, 0];
fn bfs(b: &mut [u32], i: usize, j: usize, h: usize, w: usize) {
    let mut q = std::collections::VecDeque::new();
    q.push_back((i, j, b[i<<9|j]));
    while let Some((i, j, k)) = q.pop_front() {
        if b[i<<9|j] != k { continue; }
        b[i<<9|j] = 0;
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (x, y) = (i.wrapping_add(dx), j.wrapping_add(dy));
            if x < h && y < w && b[x<<9|y] > 1 && b[x<<9|y] <= k {
                q.push_back((x, y, b[x<<9|y]));
            }
        }
    }
}
pub fn main() { read();
    let (h, w) = (next::<usize>(), next::<usize>());
    let mut b = vec![0u32; 1<<18];
    let mut v = vec![];
    for i in 0..h { for j in 0..w {
        let x = next::<u32>();
        if x > 1 {
            b[i<<9|j] = x;
            v.push((x, i, j));
        }
    }}

    v.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    let mut ans = 0;
    for (k, i, j) in v {
        if b[i<<9|j] == k {
            bfs(&mut b, i, j, h, w);
            ans += 1;
        }
    }

    println!("{}", ans);
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}