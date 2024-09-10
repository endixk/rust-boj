// BOJ 13538 [XOR Queries]
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn u32(p: &mut *const u8) -> u32 { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as u32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}

type T = u32;
#[derive(Clone, Copy, Default)] struct Node {
    l: usize, r: usize, c: T
}
struct PST {
    n: usize, tree: Vec<Node>, root: Vec<usize>
}
impl PST {
    fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        Self { n, tree: vec![Node::default()], root: vec![0] }
    }

    fn update(&mut self, s: usize, e: usize, i: usize, l: usize, r: usize, c: u32, pos: usize, x: T) {
        if s == e { self.tree[pos].c = c + x; return; }
        let m = s + e >> 1;
        if i <= m {
            let (cl, cr, cc) = (self.tree[l].l, self.tree[l].r, self.tree[l].c);
            let l = self.tree.len(); self.tree.push(Node::default());
            self.update(s, m, i, cl, cr, cc, l, x);
            self.tree[pos].l = l;
            self.tree[pos].r = r;
            self.tree[pos].c = self.tree[l].c + self.tree[r].c;
        } else {
            let (cl, cr, cc) = (self.tree[r].l, self.tree[r].r, self.tree[r].c);
            let r = self.tree.len(); self.tree.push(Node::default());
            self.update(m + 1, e, i, cl, cr, cc, r, x);
            self.tree[pos].l = l;
            self.tree[pos].r = r;
            self.tree[pos].c = self.tree[l].c + self.tree[r].c;
        }
    }

    fn insert(&mut self, i: usize) {
        let pr = *self.root.last().unwrap();
        let pos = self.tree.len();
        self.root.push(pos);
        self.tree.push(Node::default());
        self.update(0, self.n-1, i, self.tree[pr].l, self.tree[pr].r, self.tree[pr].c, pos, 1);
    }

    fn query_cnt(&self, s: usize, e: usize, l: usize, r: usize, lpos: usize, rpos: usize) -> T {
        if r < s || e < l { return 0; }
        if l <= s && e <= r { return self.tree[rpos].c - self.tree[lpos].c; }
        let m = s + e >> 1;
        self.query_cnt(s, m, l, r, self.tree[lpos].l, self.tree[rpos].l) +
            self.query_cnt(m + 1, e, l, r, self.tree[lpos].r, self.tree[rpos].r)
    }

    fn query_kth(&self, s: usize, e: usize, l: usize, r: usize, lpos: usize, rpos: usize, k: T) -> usize {
        if s == e { return s; }
        let m = s + e >> 1;
        let cnt = self.tree[self.tree[rpos].l].c - self.tree[self.tree[lpos].l].c;
        if k <= cnt {
            self.query_kth(s, m, l, r, self.tree[lpos].l, self.tree[rpos].l, k)
        } else {
            self.query_kth(m + 1, e, l, r, self.tree[lpos].r, self.tree[rpos].r, k - cnt)
        }
    }

    fn query_xor(&self, s: usize, e: usize, l: usize, r: usize, lpos: usize, rpos: usize, xor: T, dep: T) -> usize {
        if s == e { return s; }
        let m = s + e >> 1;
        let lc = self.tree[self.tree[rpos].l].c - self.tree[self.tree[lpos].l].c;
        if lc == 0 { return self.query_xor(m + 1, e, l, r, self.tree[lpos].r, self.tree[rpos].r, xor, dep - 1); }
        let rc = self.tree[self.tree[rpos].r].c - self.tree[self.tree[lpos].r].c;
        if rc == 0 { return self.query_xor(s, m, l, r, self.tree[lpos].l, self.tree[rpos].l, xor, dep - 1); }
        if xor >> (dep - 1) & 1 == 1 {
            self.query_xor(s, m, l, r, self.tree[lpos].l, self.tree[rpos].l, xor, dep - 1)
        } else {
            self.query_xor(m + 1, e, l, r, self.tree[lpos].r, self.tree[rpos].r, xor, dep - 1)
        }
    }
}

pub fn main() {
    let mut buf = input(7_999_999);
    let mut pst = PST::new(500_000);
    let dep = pst.n.trailing_zeros();
    for _ in 0..u32(&mut buf) {
        match u32(&mut buf) {
            1 => pst.insert(u32(&mut buf) as usize),
            2 => {
                let (l, r, x) = (u32(&mut buf) as usize, u32(&mut buf) as usize, u32(&mut buf));
                println!("{}", pst.query_xor(0, pst.n-1, 0, pst.n-1, pst.root[l-1], pst.root[r], x, dep));
            },
            3 => pst.root.truncate(pst.root.len() - u32(&mut buf) as usize),
            4 => {
                let (l, r, x) = (u32(&mut buf) as usize, u32(&mut buf) as usize, u32(&mut buf) as usize);
                println!("{}", pst.query_cnt(0, pst.n-1, 0, x, pst.root[l-1], pst.root[r]));
            },
            _ => {
                let (l, r, k) = (u32(&mut buf) as usize, u32(&mut buf) as usize, u32(&mut buf));
                println!("{}", pst.query_kth(0, pst.n-1, 0, pst.n-1, pst.root[l-1], pst.root[r], k));
            }
        }
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}