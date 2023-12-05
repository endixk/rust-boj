#[inline] fn get(p: &Vec<Vec<i64>>, y1: usize, y2: usize, x1: usize, x2: usize) -> i64 {
    let mut k = p[y2+1][x2+1] - p[y2+1][x1] - p[y1][x2+1] + p[y1][x1];
    k -= p[y2][x2] - p[y2][x1+1] - p[y1+1][x2] + p[y1+1][x1+1];
    k -= p[y2+1][x1+1] - p[y2+1][x1];
    k
}
pub fn main() { read();
    let (h, w) = (next::<usize>(), next::<usize>());
    let (h1, h2, w1, w2) = (next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>());
    let (h2, w2) = (if h2 == h { h-1 } else { h2 }, if w2 == w { w-1 } else { w2 });
    let mut c = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            c[i][j] = next::<i64>();
        }
    }

    let mut p = vec![vec![0; w+1]; h+1];
    for i in 0..h {
        for j in 0..w {
            p[i+1][j+1] = p[i+1][j] + p[i][j+1] - p[i][j] + c[i][j];
        }
    }

    let mut cost = 1<<60;
    for y in h1..=h2 {
        let mut q = std::collections::VecDeque::<(i64,usize)>::new();
        let mut off = 0;
        for x in (0..w-w1).rev() {
            off += c[y][x+1] + c[0][x+1];
            while !q.is_empty() && q.front().unwrap().1 > x+w2 { q.pop_front(); }
            let k = get(&p, 0, y, x, x+w1) - off;
            while !q.is_empty() && q.back().unwrap().0 >= k { q.pop_back(); }
            q.push_back((k, x+w1));
            cost = cost.min(q.front().unwrap().0 + off + p[y+1][x+1] - p[y+1][x]);
        }
    }

    if cost == 1<<60 { println!("No"); }
    else { println!("{}", cost); }
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