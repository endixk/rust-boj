// BOJ 10076 [Holiday]
#[derive(Default)] struct Node { l: u32, r: u32, cnt: u32, sum: u64 }
struct PST { n: usize, tree: Vec<Node>, root: Vec<u32> }
impl PST {
    fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        Self { n, tree: vec![Node::default()], root: vec![0] }
    }

    fn update(&mut self, s: usize, e: usize, i: usize, l: usize, r: usize, pos: usize, cnt: u32, sum: u64, x: u32) {
        if s == e {
            self.tree[pos].cnt = cnt + 1;
            self.tree[pos].sum = sum + x as u64;
            return;
        }
        let m = s + e >> 1;
        if i <= m {
            let (cl, cr) = (self.tree[l].l as usize, self.tree[l].r as usize);
            let (ccnt, csum) = (self.tree[l].cnt, self.tree[l].sum);
            let l = self.tree.len(); self.tree.push(Node::default());
            self.update(s, m, i, cl, cr, l, ccnt, csum, x);
            self.tree[pos].l = l as u32;
            self.tree[pos].r = r as u32;
            self.tree[pos].cnt = self.tree[l].cnt + self.tree[r].cnt;
            self.tree[pos].sum = self.tree[l].sum + self.tree[r].sum;
        } else {
            let (cl, cr) = (self.tree[r].l as usize, self.tree[r].r as usize);
            let (ccnt, csum) = (self.tree[r].cnt, self.tree[r].sum);
            let r = self.tree.len(); self.tree.push(Node::default());
            self.update(m + 1, e, i, cl, cr, r, ccnt, csum, x);
            self.tree[pos].l = l as u32;
            self.tree[pos].r = r as u32;
            self.tree[pos].cnt = self.tree[l].cnt + self.tree[r].cnt;
            self.tree[pos].sum = self.tree[l].sum + self.tree[r].sum;
        }
    }

    fn insert(&mut self, i: usize, x: u32) {
        let pr = *self.root.last().unwrap() as usize;
        let pos = self.tree.len();
        self.root.push(pos as u32);
        self.tree.push(Node::default());
        self.update(0, self.n-1, i, self.tree[pr].l as usize, self.tree[pr].r as usize, pos, self.tree[pr].cnt, self.tree[pr].sum, x);
    }

    fn query_ksum(&self, s: usize, e: usize, lpos: usize, rpos: usize, k: u32) -> u64 {
        if s == e { return self.tree[rpos].sum - self.tree[lpos].sum; }
        let m = s + e >> 1;
        let cnt = self.tree[self.tree[rpos].l as usize].cnt - self.tree[self.tree[lpos].l as usize].cnt;
        if k <= cnt {
            self.query_ksum(s, m, self.tree[lpos].l as usize, self.tree[rpos].l as usize, k)
        } else {
            let sum = self.tree[self.tree[rpos].l as usize].sum - self.tree[self.tree[lpos].l as usize].sum;
            sum + self.query_ksum(m + 1, e, self.tree[lpos].r as usize, self.tree[rpos].r as usize, k - cnt)
        }
    }
}

fn dnco(dp: &mut Vec<u64>, ls: usize, le: usize, rs: usize, re: usize, pst: &PST, s: usize, d: usize) {
    if ls > le { return; }
    let lm = ls + le >> 1;
    let st = rs.max(lm);
    if st + s - 2 * lm >= d { dnco(dp, lm+1, le, rs, re, pst, s, d); return; }
    let ed = re.min(d + 2 * lm - s - 1);

    let (mut max, mut maxr) = (0, st);
    for r in st..=ed {
        let x = pst.query_ksum(0, pst.n-1, pst.root[lm-1] as usize, pst.root[r.max(s)] as usize, (d + 2 * lm - s - r) as u32);
        if x > max { max = x; maxr = r; }
    }

    dp[lm] = max;
    dnco(dp, ls, lm-1, rs, maxr, pst, s, d);
    dnco(dp, lm+1, le, maxr, re, pst, s, d);
}

pub fn main() { read();
    let (n, s, d) = (next::<usize>(), next::<usize>() + 1, next::<usize>());
    let a = (0..n).map(|_| next::<u32>()).collect::<Vec<_>>();

    let mut v = a.iter().enumerate().map(|(i, &x)| (x, i)).collect::<Vec<_>>();
    v.sort_unstable(); v.reverse();
    let mut w = vec![(0, 0); n];
    for (i, (x, k)) in v.into_iter().enumerate() { w[k] = (x, i); }
    let mut pst = PST::new(n);
    for (x, i) in w { pst.insert(i, x); }
    let mut dp = vec![0; s+1];
    dnco(&mut dp, 1, s, 1, n, &pst, s, d);
    let ans = *dp.iter().max().unwrap();

    v = a.iter().rev().enumerate().map(|(i, &x)| (x, i)).collect::<Vec<_>>();
    v.sort_unstable(); v.reverse();
    w = vec![(0, 0); n];
    for (i, (x, k)) in v.into_iter().enumerate() { w[k] = (x, i); }
    pst = PST::new(n);
    for (x, i) in w { pst.insert(i, x); }
    let s = n - s + 1;
    dp = vec![0; s+1];
    dnco(&mut dp, 1, s, 1, n, &pst, s, d);

    println!("{}", ans.max(*dp.iter().max().unwrap()));
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