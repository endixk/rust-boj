// BOJ 31427 [Road Network and Queries]
// snippet from https://github.com/satylogin/cp-lib/blob/main/src/algo/next_permutation.rs
fn next_permutation<T>(arr: &mut [T]) -> bool where T: Ord {
    use std::cmp::Ordering;
    let la = match arr.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => { arr.reverse();return false; }
    };
    let sw = arr[la + 1..]
        .binary_search_by(|n| match arr[la].cmp(n) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        }).unwrap_err();
    arr.swap(la, la + sw);
    arr[la + 1..].reverse();
    true
}
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
fn mst(e: &mut Vec<(usize, usize, usize)>, p: &[u8], n: usize) -> [usize; 5] {
    e.sort_unstable_by_key(|x| p[x.2]);
    let mut root = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut ret = [0; 5];
    for (u, v, t) in e {
        if find(&mut root, *u) != find(&mut root, *v) {
            union(&mut root, &mut rank, *u, *v);
            ret[*t] += 1;
        }
    }
    ret
}
pub fn main() { read();
    let (n, m, q) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut e = vec![];
    for _ in 0..m {
        let (u, v, t) = (next::<usize>() - 1, next::<usize>() - 1, next::<char>());
        e.push((u, v, t as usize - b'A' as usize));
    }
    let mut p = vec![0, 1, 2, 3, 4];
    let mut msts = vec![];
    loop {
        msts.push(mst(&mut e, &p, n));
        if !next_permutation(&mut p) { break; }
    }

    for _ in 0..q {
        let (a, b, c, d, e) = (next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>());
        let mut ans = usize::MAX;
        for &[na, nb, nc, nd, ne] in &msts {
            ans = ans.min(a * na + b * nb + c * nc + d * nd + e * ne);
        }
        println!("{}", ans);
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