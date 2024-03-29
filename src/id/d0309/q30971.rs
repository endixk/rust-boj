// BOJ 30971 [Bibimbap]
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
pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<u16>());
    let a = (0..n).map(|_| next::<u16>()).collect::<Vec<_>>();
    let b = (0..n).map(|_| next::<u16>()).collect::<Vec<_>>();
    let c = (0..n).map(|_| next::<u16>()).collect::<Vec<_>>();

    let mut arr = (0..n).collect::<Vec<_>>();
    let mut ans = 0;
    loop {
        let (mut sum, mut flag) = (0, true);
        for i in 1..n {
            if c[arr[i-1]] * c[arr[i]] > k { flag = false; break; }
            sum += a[arr[i-1]] * b[arr[i]];
        }
        if flag { ans = ans.max(sum); }
        if !next_permutation(&mut arr) { break; }
    }
    if ans == 0 { println!("-1"); }
    else { println!("{}", ans); }
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