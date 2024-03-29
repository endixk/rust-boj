// BOJ 27317 [Fluffy Fox Tail]
pub fn main() { read();
    let (n, m, k, t) = (next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>());
    let mut v = vec![(0, 0)];
    let (s, e) = (next::<usize>(), next::<usize>());
    if s == 0 { v[0].1 = e; } else { v.push((s, e)); }
    for _ in 1..n { v.push((next::<usize>(), next::<usize>())); }
    if v.last().unwrap().1 != m { v.push((m, m)); }

    let c = (1..v.len()).map(|i| (v[i].0 - v[i-1].1 + t - 1) / t).collect::<Vec<_>>();
    assert!(c.iter().all(|&x| x > 0));
    let z = c.iter().sum::<usize>();
    if z <= k { println!("{}", m); return; }

    let (mut i, mut x, mut j, mut y) = (0, 0, 0, 0);
    let mut sum = v[0].1 - v[0].0;
    for _ in 0..k {
        y += 1;
        if y == c[j] {
            sum += if (v[j+1].0 - v[j].1) % t == 0 { t } else { (v[j+1].0 - v[j].1) % t } + v[j+1].1 - v[j+1].0; j += 1; y = 0;
        } else { sum += t; }
    }

    let mut ans = sum;
    for _ in k..z {
        y += 1;
        if y == c[j] {
            sum += if (v[j+1].0 - v[j].1) % t == 0 { t } else { (v[j+1].0 - v[j].1) % t } + v[j+1].1 - v[j+1].0; j += 1; y = 0;
        } else { sum += t; }

        if x == 0 {
            sum -= v[i].1 - v[i].0 + if i == j { t } else if (v[i+1].0 - v[i].1) % t == 0 { t } else { (v[i+1].0 - v[i].1) % t };
        } else { sum -= t; }
        x += 1; if x == c[i] { i += 1; x = 0; }

        ans = ans.max(sum);
    }

    assert!(ans < m);
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