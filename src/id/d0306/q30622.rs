// BOJ 30622 [Rush & Slash]
// Supported by GitHub Copilot

fn fill(v: &mut Vec<bool>, w: &Vec<(i32, i32)>, adj: &Vec<Vec<usize>>, cur: usize) -> i32 {
    let mut ret = i32::MAX;
    let mut q = std::collections::VecDeque::new();
    q.push_back(cur); v[cur] = true;
    while let Some(cur) = q.pop_front() {
        ret = ret.min(w[cur].0.abs() + w[cur].1.abs());
        for &nxt in adj[cur].iter() {
            if !v[nxt] {
                v[nxt] = true;
                q.push_back(nxt);
            }
        }
    }
    ret
}
pub fn main() { read();
    let n = next::<usize>();
    let mut map = std::collections::HashMap::new();
    let mut w = vec![];
    for i in 0..n {
        let (x, y) = (next::<i32>(), next::<i32>());
        map.insert((x, y), i);
        w.push((x, y));
    }

    let mut adj = vec![vec![]; n];
    const DX: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];
    const DY: [i32; 8] = [1, 1, 0, -1, -1, -1, 0, 1];
    for i in 0..n {
        let (x, y) = w[i];
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            if let Some(&j) = map.get(&(x + dx, y + dy)) {
                adj[i].push(j);
            }
        }
    }

    let mut v = vec![false; n];
    let mut d = vec![];
    for i in 0..n {
        if !v[i] { d.push(fill(&mut v, &w, &adj, i) as i64); }
    }
    d.sort_unstable();

    println!("{}", d.iter().sum::<i64>() * 2 - d.last().unwrap());
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