// BOJ 13309 [Tree]
type T = bool;
struct SegTree {
    n: usize,
    v: Vec<T>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), v: vec![false; n.next_power_of_two()<<1] }
    }
    fn update(&mut self, mut i: usize, _: T) {
        i |= self.n;
        self.v[i] = true;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] | self.v[i<<1|1];
        }
    }
    fn query(&self, mut l: usize, mut r: usize) -> T {
        l |= self.n; r |= self.n;
        let mut ans = false;
        while l <= r {
            if l & 1 == 1 { ans |= self.v[l]; l += 1; }
            if r & 1 == 0 { ans |= self.v[r]; r -= 1; }
            l >>= 1; r >>= 1;
        }
        ans
    }
}

// DFS 1: convert graph to tree
fn dfs_convert(v: usize, vis: &mut Vec<bool>, ud: &mut Vec<Vec<usize>>, bd: &Vec<Vec<usize>>) {
    vis[v] = true;
    for &u in &bd[v] {
        if !vis[u] {
            ud[v].push(u);
            dfs_convert(u, vis, ud, bd);
        }
    }
}
// DFS 2: fix subtree
fn dfs_subtree(v: usize, sz: &mut Vec<usize>, dep: &mut Vec<usize>, par: &mut Vec<usize>, ud: &mut Vec<Vec<usize>>) {
    sz[v] = 1;
    for i in 0..ud[v].len() {
        let u = ud[v][i];
        (dep[u], par[u]) = (dep[v] + 1, v);
        dfs_subtree(u, sz, dep, par, ud);
        sz[v] += sz[u];
        if sz[u] > sz[ud[v][0]] {
            ud[v].swap(0, i);
        }
    }
}
// DFS 3: decompose tree
fn dfs_decompose(v: usize, it: &mut usize, vin: &mut Vec<usize>, vout: &mut Vec<usize>, vtop: &mut Vec<usize>, ud: &Vec<Vec<usize>>) {
    vin[v] = *it; *it += 1;
    for &u in &ud[v] {
        vtop[u] = if u == ud[v][0] { vtop[v] } else { u };
        dfs_decompose(u, it, vin, vout, vtop, ud);
    }
    vout[v] = *it;
}

fn hld_update(v: usize, _: T, seg: &mut SegTree, vin: &[usize]) {
    seg.update(vin[v], true);
}
fn hld_query(u: usize, v: usize, seg: &SegTree,
             vin: &[usize], vtop: &[usize], dep: &[usize], par: &[usize]) -> T {
    let mut ret = false;
    let (mut u, mut v) = (u, v);
    while vtop[u] != vtop[v] {
        (u, v) = if dep[vtop[u]] < dep[vtop[v]] { (v, u) } else { (u, v) };
        ret |= seg.query(vin[vtop[u]], vin[u]);
        u = par[vtop[u]];
    }
    (u, v) = if dep[u] > dep[v] { (v, u) } else { (u, v) };
    ret |= seg.query(vin[u], vin[v]);
    ret
}

pub fn main() { read();
    let m = next::<usize>();
    let (n, q) = (m<<1, next::<usize>());
    let mut bd = vec![vec![]; n];
    bd[0].push(m); bd[m].push(0);
    for i in 1..m {
        let p = next::<usize>() - 1;
        bd[p].push(i+m); bd[i+m].push(p);
        bd[i+m].push(i); bd[i].push(i+m);
    }

    let mut ud = vec![vec![]; n];
    dfs_convert(0, &mut vec![false; n], &mut ud, &bd);

    let mut sz = vec![0; n];
    let mut dep = vec![0; n];
    let mut par = vec![0; n];
    dfs_subtree(0, &mut sz, &mut dep, &mut par, &mut ud);

    let mut vin = vec![0; n];
    let mut vout = vec![0; n];
    let mut vtop = vec![0; n];
    let mut it = 0;
    dfs_decompose(0, &mut it, &mut vin, &mut vout, &mut vtop, &ud);

    let mut seg = SegTree::new(n);
    for _ in 0..q {
        let (u, v, t) = (next::<usize>() - 1, next::<usize>() - 1, next::<u8>() == 0);
        if t {
            println!("{}", if hld_query(u, v, &seg, &vin, &vtop, &dep, &par) { "NO" } else { "YES" });
        } else {
            if hld_query(u, v, &seg, &vin, &vtop, &dep, &par) {
                println!("NO");
                hld_update(v+m, true, &mut seg, &vin);
            } else {
                println!("YES");
                hld_update(u+m, true, &mut seg, &vin);
            }
        }
    }
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