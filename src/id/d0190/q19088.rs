// BOJ 19088 [Biggest Number]
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn ptr(p: &mut *const u8) -> u32 { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as u32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
const MOD: u64 = 1_000_000_007;
type T = u64;
struct SegTree { n: usize, v: Vec<T>, c: Vec<usize> }
impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            n: n.next_power_of_two(),
            v: vec![T::default(); n.next_power_of_two()<<1],
            c: vec![0; n.next_power_of_two()<<1]
        }
    }
    fn update(&mut self, mut i: usize, x: T, add: bool, dmod: &[T]) {
        i |= self.n;
        self.v[i] = if add { (self.v[i] * dmod[1] + x) % MOD } else { (self.v[i] + MOD - (x * dmod[self.c[i] - 1]) % MOD) % MOD };
        self.c[i] = if add { self.c[i] + 1 } else { self.c[i] - 1 };
        while i > 1 {
            i >>= 1;
            self.c[i] = self.c[i<<1] + self.c[i<<1|1];
            self.v[i] = (self.v[i<<1|1] * dmod[self.c[i<<1]] + self.v[i<<1]) % MOD;
        }
    }
}
pub fn main() {
    let mut p = input(2777777);
    let (n, q, d) = (ptr(&mut p) as usize, ptr(&mut p) as usize, ptr(&mut p) as u64);
    let mut a = vec![];
    let mut v = vec![];

    for _ in 0..n {
        let x = ptr(&mut p) as u64;
        a.push(x); v.push(x);
    }
    let mut qry = vec![];
    for _ in 0..q {
        let (i, x) = (ptr(&mut p) as usize - 1, ptr(&mut p) as u64);
        qry.push((i, x)); v.push(x);
    }

    let mut map = std::collections::HashMap::new();
    v.sort_unstable(); v.dedup();
    for (i, &x) in v.iter().enumerate() { map.insert(x, i); }

    let mut dmod = vec![0; n+1];
    dmod[0] = 1;
    for i in 1..=n { dmod[i] = (dmod[i-1] * d) % MOD; }

    let mut seg = SegTree::new(v.len()+1);
    for i in 0..n { seg.update(map[&a[i]], a[i], true, &dmod); }
    println!("{}", seg.v[1]);
    for (i, x) in qry {
        seg.update(map[&a[i]], a[i], false, &dmod);
        a[i] = x;
        seg.update(map[&a[i]], x, true, &dmod);
        println!("{}", seg.v[1]);
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}