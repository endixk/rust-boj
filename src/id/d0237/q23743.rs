// BOJ 23743 [Room Escape]
// Supported by GitHub Copilot

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x { x }
    else {
        root[x] = find(root, root[x]);
        root[x]
    }
}
fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let (x, y) = (find(root, x), find(root, y));
    if x == y { return; }
    if rank[x] < rank[y] {
        root[x] = y;
    } else {
        root[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}

pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut e = Vec::new();
    for _ in 0..m {
        let (a, b, c) = (next::<usize>(), next::<usize>(), next::<usize>());
        e.push((c, a, b));
    }
    for i in 0..n {
        let t = next::<usize>();
        e.push((t, 0, i+1));
    }
    e.sort_unstable();

    let mut root = (0..=n).collect::<Vec<usize>>();
    let mut rank = vec![0; n+1];
    let mut ans = 0;
    for (c, a, b) in e {
        if find(&mut root, a) != find(&mut root, b) {
            union(&mut root, &mut rank, a, b);
            ans += c;
        }
    }

    println!("{}", ans);
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