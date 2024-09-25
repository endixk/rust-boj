// BOJ 16124 [I Am Happy]
const MOD: u64 = 998_244_353;
const DEF: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
struct SegTree { n: usize, v: Vec<[u32; 10]>, lazy: Vec<[u8; 10]> }
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![[0; 10]; m<<1], lazy: vec![DEF; m<<1] }
    }

    fn build(&mut self, a: &[u8], pmod: &[u64]) {
        let mut len = vec![0; self.n<<1];
        for i in 0..a.len() {
            self.v[self.n + i][a[i] as usize] = 1;
            len[self.n + i] = 1;
        }
        for i in (1..self.n).rev() {
            for j in 0..10 {
                self.v[i][j] = ((self.v[i<<1][j] as u64 * pmod[len[i<<1|1]] % MOD + self.v[i<<1|1][j] as u64) % MOD) as u32;
            }
            len[i] = len[i<<1] + len[i<<1|1];
        }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == DEF { return; }
        let mut nv = [0; 10];
        for i in 0..10 {
            nv[self.lazy[x][i] as usize] = (nv[self.lazy[x][i] as usize] + self.v[x][i]) % MOD as u32;
        }
        self.v[x] = nv;
        if s < e {
            let (mut llazy, mut rlazy) = (self.lazy[x<<1], self.lazy[x<<1|1]);
            for i in 0..10 {
                llazy[i] = self.lazy[x][self.lazy[x<<1][i] as usize];
                rlazy[i] = self.lazy[x][self.lazy[x<<1|1][i] as usize];
            }
            self.lazy[x<<1] = llazy;
            self.lazy[x<<1|1] = rlazy;
        }
        self.lazy[x] = DEF;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, fr: u8, to: u8, pmod: &[u64]) {
        if fr == to { return; }
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            for i in 0..10 {
                if self.lazy[x][i] == fr { self.lazy[x][i] = to; }
            }
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, fr, to, pmod);
            self.update(x<<1|1, m+1, e, l, r, fr, to, pmod);
            for i in 0..10 {
                self.v[x][i] = ((self.v[x<<1][i] as u64 * pmod[e-m] % MOD + self.v[x<<1|1][i] as u64) % MOD) as u32;
            }
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, pmod: &[u64]) -> u64 {
        self.propagate(x, s, e);
        if r < s || e < l { return 0; }
        if l <= s && e <= r {
            (0..10).fold(0, |acc, i| (acc + self.v[x][i] as u64 * i as u64) % MOD)
        } else {
            let m = (s + e) >> 1;
            (self.query(x<<1, s, m, l, r, pmod) * pmod[e.min(r).saturating_sub(m)] % MOD + self.query(x<<1|1, m+1, e, l, r, pmod)) % MOD
        }
    }
}

pub fn main() { read();
    let s = next::<String>().as_bytes().iter().map(|&c| c - b'0').collect::<Vec<_>>();
    let n = s.len();
    let mut pmod = vec![1; n];
    for i in 1..n { pmod[i] = pmod[i-1] * 10 % MOD; }
    let mut seg = SegTree::new(n);
    seg.build(&s, &pmod);

    for _ in 0..next() {
        match next::<u8>() {
            1 => {
                let (l, r, fr, to) = (next::<usize>(), next::<usize>(), next::<u8>(), next::<u8>());
                seg.update(1, 1, seg.n, l, r, fr, to, &pmod);
            },
            _ => {
                let (l, r) = (next::<usize>(), next::<usize>());
                println!("{}", seg.query(1, 1, seg.n, l, r, &pmod));
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