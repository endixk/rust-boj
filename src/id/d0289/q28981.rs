// BOJ 28981 [Restoring the Number]
// Supported by GitHub Copilot

#[inline] fn gen(a: &[char]) -> i64 {
    let mut ret = 0;
    for &x in a {
        ret = ret * 10 + (x as u8 - b'0') as i64;
    }
    ret
}
fn get(g: &mut Vec<i64>, a: &mut [char], i: usize, n: usize, lead: bool) {
    if i == n { g.push(gen(a)); return; }
    if a[i] == '?' {
        for j in 0..10 {
            if lead && i == 0 && j == 0 { continue; }
            a[i] = (b'0' + j) as char;
            get(g, a, i+1, n, false);
            a[i] = '?';
        }
    } else { get(g, a, i+1, n, false); }
}
pub fn main() { read();
    let (n, m) = (next::<String>(), next::<i64>());
    let n = n.chars().collect::<Vec<_>>();
    let x = n.len();

    let mut a = n.iter().take(x/2).copied().collect::<Vec<_>>();
    let mut g = vec![];
    get(&mut g, &mut a, 0, x/2, true);

    let mut b = n.iter().skip(x/2).copied().collect::<Vec<_>>();
    let mut h = vec![];
    get(&mut h, &mut b, 0, x-x/2, false);

    let g = g.iter().map(|&k| k * 10i64.pow((x-x/2) as u32) % m).collect::<Vec<_>>();
    let mut h = h.iter().map(|&k| k % m).collect::<Vec<_>>();
    h.sort_unstable();
    h.push(h[0] + m);

    let mut ans = m;
    for p in g {
        let k = h.partition_point(|&r| r < m-p);
        if h[k] == m-p { ans = 0; break; }
        else if ans > p + h[k] - m { ans = p + h[k] - m; }
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