// BOJ 7453 [4 Values whose Sum is 0]
pub fn main() { read();
    let n = next::<usize>();
    let (mut a, mut b, mut c, mut d) = (vec![], vec![], vec![], vec![]);
    for _ in 0..n {
        a.push(next::<i32>()); b.push(next::<i32>()); c.push(next::<i32>()); d.push(next::<i32>());
    }

    let (mut v, mut w) = (vec![], vec![]);
    for i in 0..n { for j in 0..n {
        v.push(a[i] + b[j]);
        w.push(c[i] + d[j]);
    }}
    v.sort_unstable();
    w.sort_unstable();

    let (mut i, mut j) = (0, w.len() as i32 - 1);
    let mut ans = 0;
    while i < v.len() as i32 && j >= 0 {
        let (mut x, mut y) = (i as usize, j as usize);
        if v[x] + w[y] == 0 {
            let (mut cnt1, mut cnt2) = (1, 1);
            while x + 1 < v.len() && v[x] == v[x + 1] { cnt1 += 1; x += 1; }
            while y > 0 && w[y] == w[y - 1] { cnt2 += 1; y -= 1; }
            ans += cnt1 as i64 * cnt2 as i64;
            i = x as i32 + 1;
            j = y as i32 - 1;
        } else if v[x] + w[y] < 0 { i += 1; }
        else { j -= 1; }

    }
    println!("{}", ans);
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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