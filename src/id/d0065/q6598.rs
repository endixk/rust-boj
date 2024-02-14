// BOJ 6598 [Arbitrage]
pub fn main() { read();
    for tc in 1.. {
        let n = next::<usize>();
        if n == 0 {break;}

        let mut map = std::collections::HashMap::new();
        (0..n).for_each(|i| { map.insert(next::<String>(), i); });
        let m = next::<usize>();

        let mut e = vec![];
        (0..m).for_each(|_| {
            let (u, w, v) = (next::<String>(), next::<f64>(), next::<String>());
            e.push((map[&u], map[&v], w));
        });

        let mut c = vec![0.0; n];
        c[0] = 1.0;
        for _ in 0..n {
            let mut nc = c.clone();
            for &(u, v, w) in &e {
                if nc[v].partial_cmp(&(c[u] * w)).unwrap() == std::cmp::Ordering::Less {
                    nc[v] = c[u] * w;
                }
            }
            c = nc;
        }

        println!("Case {}: {}", tc, if c[0] > 1.0 + 1e-9 { "Yes" } else { "No" });
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