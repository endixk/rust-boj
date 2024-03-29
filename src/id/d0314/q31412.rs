// BOJ 31412 [Organizing a Supply Depot]
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
fn go(p: &[u64], b: &[u64], x: u64) -> bool {
    let mut s = 0;
    for &k in b {
        let i = p.partition_point(|&y| y <= x * k + p[s]);
        if i == p.len() { return true; }
        if i == s + 1 { return false; }
        s = i - 1;
    }
    false
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut p = vec![0; n+1];
    for i in 1..=n { p[i] = p[i-1] + next::<u64>(); }
    let mut b = (0..m).map(|_| next::<u64>()).collect::<Vec<_>>();
    b.sort();

    let mut ans = p[n];
    loop {
        let (mut l, mut r) = (1, p[n]);
        while l < r {
            let x = (l + r) / 2;
            if go(&p, &b, x) { r = x; }
            else { l = x + 1; }
        }
        ans = ans.min(l);
        if !next_permutation(&mut b) { break; }
    }
    println!("{}", ans);
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