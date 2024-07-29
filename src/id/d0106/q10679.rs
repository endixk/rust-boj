// BOJ 10679 [Broken Audio Signal]
const UNK: i32 = 1_000_000_001;
const INF: i32 = 1_000_000_002;
pub fn main() { read();
    'x: loop {
        let n = next::<usize>();
        if n == 0 { break; }

        let mut v = vec![0; n+2];
        v[0] = INF;
        v[n+1] = if n & 1 == 0 { -INF } else { INF };
        for i in 1..=n { v[i] = match next::<String>().as_str() {
            "x" => UNK * if i & 1 == 0 { 1 } else { -1 },
            s => s.parse().unwrap() };
        }

        let (mut l, mut r) = (-INF, INF);
        for i in 1..=n {
            if v[i].abs() == UNK {
                if i & 1 == 1 { r = r.min(v[i-1] - 1).min(v[i+1] - 1); }
                else { l = l.max(v[i-1] + 1).max(v[i+1] + 1); }
            }
        }

        if l > r { println!("none"); continue; }
        for i in 1..=n { if v[i].abs() == UNK { v[i] = l; }}
        for i in 1..=n {
            if i & 1 == 0 && (v[i] <= v[i-1] || v[i] <= v[i+1]) { println!("none"); continue 'x; }
            if i & 1 == 1 && (v[i] >= v[i-1] || v[i] >= v[i+1]) { println!("none"); continue 'x; }
        }
        println!("{}", if l == r { l.to_string() } else { "ambiguous".to_string() });
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
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