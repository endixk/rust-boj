// BOJ 6589 [Heavy Cargo]
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
    'tc: for i in 1.. {
        let (n, r) = (next::<usize>(), next::<usize>());
        if n == 0 && r == 0 { break; }

        let mut map = std::collections::HashMap::new();
        let mut x = 0usize;
        let mut e = vec![];
        for _ in 0..r {
            let (u, v, w) = (next::<String>(), next::<String>(), next::<usize>());
            let u = *map.entry(u).or_insert_with(|| { x += 1; x - 1 });
            let v = *map.entry(v).or_insert_with(|| { x += 1; x - 1 });
            e.push((w, u, v));
        }
        e.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let (src, dst) = (map[&next::<String>()], map[&next::<String>()]);
        let mut root = (0..n).collect::<Vec<_>>();
        let mut rank = vec![0; n];
        for (w, u, v) in e {
            union(&mut root, &mut rank, u, v);
            if find(&mut root, src) == find(&mut root, dst) {
                println!("Scenario #{}", i);
                println!("{} tons", w);
                println!();
                continue 'tc;
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}