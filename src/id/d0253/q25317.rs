// BOJ 25317 [functionx]
fn gcd(mut a: i128, mut b: i128) -> i128 { while b != 0 { let t = b; b = a % b; a = t; } a }
#[derive(Clone, Copy, Hash)] struct Frac { n: i128, d: i128, sign: bool }
impl Frac {
    fn new(n: i128, d: i128) -> Self {
        if n == 0 { return Frac { n: 0, d: 1, sign: true }; }
        let neg = n != 0 && ((n < 0) ^ (d < 0));
        let (n, d) = (n.abs(), d.abs());
        let g = gcd(n, d);
        Frac { n: n/g, d: d/g, sign: !neg }
    }
}
impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool { (self.sign == other.sign) && (self.n * other.d == self.d * other.n) }
}
impl Eq for Frac {}
impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for Frac {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.sign != other.sign { return self.sign.cmp(&other.sign); }
        if self.sign {
            (self.n * other.d).cmp(&(self.d * other.n))
        } else {
            (self.d * other.n).cmp(&(self.n * other.d))
        }
    }
}

type T = u32;
struct SegTree { n: usize, v: Vec<T>, }
impl SegTree {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), v: vec![T::default(); n.next_power_of_two()<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i |= self.n;
        self.v[i] += x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[i<<1|1];
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> T {
        l |= self.n; r |= self.n;
        let mut ans = T::default();
        while l <= r {
            if l & 1 == 1 { ans += self.v[l]; l += 1; }
            if r & 1 == 0 { ans += self.v[r]; r -= 1; }
            l >>= 1; r >>= 1;
        }
        ans
    }
}

pub fn main() { read();
    let mut qry = vec![];
    let mut fracs = vec![];
    for _ in 0..next() {
        let t = next::<u8>();
        if t == 1 {
            let (a, b) = (next::<i128>(), next::<i128>());
            if a == 0 {
                qry.push((t, 0, b));
            } else {
                qry.push((t, a, b));
                fracs.push(Frac::new(b, -a));
            }
        } else {
            let c = next::<i128>();
            qry.push((t, c, 1));
            fracs.push(Frac::new(c, 1));
        }
    }

    fracs.sort_unstable(); fracs.dedup();
    let mut map = std::collections::HashMap::new();
    for (i, &f) in fracs.iter().enumerate() {
        map.insert(f, i);
    }

    let mut seg = SegTree::new(fracs.len());
    let (mut odd, mut neg, mut zero) = (false, false, false);
    for (t, a, b) in qry {
        if t == 1 {
            if a == 0 {
                if b < 0 { neg = !neg; }
                else if b == 0 { zero = true; }
                continue;
            }
            odd = !odd;
            if a < 0 { neg = !neg; }
            seg.update(map[&Frac::new(b, -a)], 1);
        } else {
            if zero { println!("0"); continue; }
            let i = map[&Frac::new(a, 1)];
            if seg.query(i, i) > 0 { println!("0"); }
            else {
                let mut f = odd ^ neg;
                if seg.query(0, i) & 1 == 1 { f = !f; }
                println!("{}", if f { "-" } else { "+" });
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