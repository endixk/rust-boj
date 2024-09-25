// BOJ 18123 [Parallel Universe]
fn sz(adj: &Vec<Vec<usize>>, u: usize, p: usize) -> usize {
    let mut ret = 1;
    for &v in adj[u].iter() {
        if v == p { continue; }
        ret += sz(adj, v, u);
    }
    ret
}
fn cent(adj: &Vec<Vec<usize>>, u: usize, n: usize) -> (usize, Option<usize>) {
    let (mut v, mut c) = (0, 0);
    for &x in adj[u].iter() {
        let z = sz(&adj, x, u);
        if z > c { c = z; v = x; }
    }
    if c << 1 > n { return cent(adj, v, n); }
    if c << 1 == n { (u, Some(v)) } else { (u, None) }
}
fn hash(adj: &Vec<Vec<usize>>, u: usize, p: usize) -> (u64, u32) {
    let mut sub = vec![];
    for &v in adj[u].iter() {
        if v == p { continue; }
        sub.push(hash(adj, v, u));
    }
    sub.sort_unstable_by_key(|x| x.0);

    let (mut ret, mut s) = (0, 0);
    for (h, l) in sub {
        ret <<= l; ret |= h; s += l;
    }
    (2 << s | ret << 1, s + 2)
}
pub fn main() { read();
    let mut v = vec![];
    for _ in 0..next() {
        let n = next::<usize>();
        let mut adj = vec![vec![]; n];
        for _ in 1..n {
            let (u, v) = (next::<usize>(), next::<usize>());
            adj[u].push(v); adj[v].push(u);
        }
        let (c1, c2) = cent(&mut adj, 0, n);
        let mut h = hash(&adj, c1, c1).0;
        if let Some(c) = c2 { h = h.min(hash(&adj, c, c).0); }
        v.push(h);
    }
    v.sort_unstable(); v.dedup();
    println!("{}", v.len());
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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