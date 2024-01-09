// BOJ 30880 [Query Isn't ROCK]
const M: u64 = 1_000_000_007;
#[derive(Clone, Copy, Default)] struct Rock {
    r: u64, ro: u64, roc: u64, rock: u64,
    o: u64, c: u64, k: u64, oc: u64, ck: u64, ock: u64,
}
fn sum(x: &Rock, y: &Rock, lx: u64) -> Rock {
    Rock {
        r: (x.r + lx * y.r) % M,
        ro: (x.ro + x.r * y.o + lx * y.ro) % M,
        roc: (x.roc + x.r * y.oc + x.ro * y.c + lx * y.roc) % M,
        rock: (x.rock + x.r * y.ock + x.ro * y.ck + x.roc * y.k + lx * y.rock) % M,
        o: (x.o + y.o) % M,
        c: (x.c + y.c) % M,
        k: (x.k + y.k) % M,
        oc: (x.oc + y.oc + x.o * y.c) % M,
        ck: (x.ck + y.ck + x.c * y.k) % M,
        ock: (x.ock + y.ock + x.oc * y.k + x.o * y.ck) % M,
    }
}
struct SegTree {
    n: usize, rock: [Rock; 1<<19],
}
impl SegTree {
    fn new(n: usize) -> Self {
        let n = n.next_power_of_two();
        Self { n, rock: [Rock::default(); 1<<19] }
    }
    fn update(&mut self, mut i: usize, x: u8) {
        i += self.n;
        let mut l = 2;
        self.rock[i].r = if x == b'R' { 1 } else { 0 };
        self.rock[i].o = if x == b'O' { 1 } else { 0 };
        self.rock[i].c = if x == b'C' { 1 } else { 0 };
        self.rock[i].k = if x == b'K' { 1 } else { 0 };
        while i > 1 {
            i >>= 1;
            let (x, y) = (i<<1, i<<1|1);
            self.rock[i] = sum(&self.rock[x], &self.rock[y], l);
            l = (l * l) % M;
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> Rock {
        l += self.n; r += self.n;
        let (mut rx, mut ry) = (Rock::default(), Rock::default());
        let (mut ax, mut lx) = (1, 2);
        while l <= r {
            if l & 1 == 1 {
                rx = sum(&rx, &self.rock[l], ax);
                l += 1; ax = (ax * lx) % M;
            }
            if r & 1 == 0 {
                ry = sum(&self.rock[r], &ry, lx);
                r -= 1;
            }
            l >>= 1; r >>= 1; lx = (lx * lx) % M;
        }
        sum(&rx, &ry, ax)
    }
}
pub fn main() { read();
    let n = next::<usize>();
    let mut seg = SegTree::new(n);
    for (i, c) in next::<String>().into_bytes().into_iter().enumerate() {
        seg.update(i, c);
    }
    for _ in 0..next() {
        let q = next::<u8>();
        if q == 1 {
            let (i, c) = (next::<usize>(), next::<char>());
            seg.update(i-1, c as u8);
        } else {
            let (l, r) = (next::<usize>(), next::<usize>());
            println!("{}", seg.query(l-1, r-1).rock);
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