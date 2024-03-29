// BOJ 30627 [March 1st]
// Supported by GitHub Copilot

fn hash(t1: usize, p1: usize, t2: usize, p2: usize) -> usize {
    t1 << 9 | p1 << 7 | t2 << 2 | p2
}
fn go(t1: usize, p1: usize, t2: usize, p2: usize) -> (usize, usize) {
    let r = if p1 + p2 == 1 { 2 }  else if p1 + p2 == 2 { 1 } else { 0 };
    (hash(t1 - 1, r, t2, p2), hash(t1, p1, t2 - 1, r))
}
pub fn main() { read();
    let (_, t1, t2) = (next::<usize>(), next::<usize>(), next::<usize>());
    let (x, y, _) = (next::<usize>(), next::<usize>(), next::<usize>());
    let d1 = if x == 1 { 0 } else if y == 1 { 1 } else { 2 };
    let d2 = if x == 2 { 0 } else if y == 2 { 1 } else { 2 };

    let ori = hash(20, 0, 20, 1);
    let dst = hash(t1/5, d1, t2/5, d2);
    let mut v = vec![false; 1 << 14];
    let mut t = vec![0; 1 << 14];
    let mut q = std::collections::VecDeque::new();
    v[ori] = true; q.push_back(ori);
    while let Some(h) = q.pop_front() {
        if h == dst { break; }
        let (t1, p1, t2, p2) = (h >> 9, h >> 7 & 3, h >> 2 & 31, h & 3);
        if t1 == 0 || t2 == 0 { continue; }
        let (h1, h2) = go(t1, p1, t2, p2);
        if !v[h1] { v[h1] = true; t[h1] = h; q.push_back(h1); }
        if !v[h2] { v[h2] = true; t[h2] = h; q.push_back(h2); }
    }

    let mut ans = Vec::new();
    let mut h = dst;
    while h != 0 { ans.push(h); h = t[h]; }
    ans.reverse();

    if ans.len() == 1 {
        if ori == dst { println!("0"); }
        else { println!("-1"); }
    } else {
        println!("{}", ans.len() - 1);
        for i in 1..ans.len() {
            let (t1, p1, _, p2) = (ans[i-1] >> 9, ans[i-1] >> 7 & 3, ans[i-1] >> 2 & 31, ans[i-1] & 3);
            let (t3, p3, _, p4) = (ans[i]   >> 9, ans[i]   >> 7 & 3, ans[i]   >> 2 & 31, ans[i]   & 3);
            if t1 > t3 {
                println!("{} {}", p1 + 1, p3 + 1);
            } else {
                println!("{} {}", p2 + 1, p4 + 1);
            }
        }
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