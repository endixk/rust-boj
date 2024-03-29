// BOJ 31219 [World Tour]
#[derive(Eq, PartialEq, Clone, Copy)] struct Point { x: i64, y: i64 }
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
fn dist(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x) as f64).hypot((a.y - b.y) as f64)
}
struct Line { a: Point, b: Point }
fn intersect(l1: &Line, l2: &Line) -> bool {
    ccw(&l1.a, &l1.b, &l2.a) * ccw(&l1.a, &l1.b, &l2.b) <= 0 && ccw(&l2.a, &l2.b, &l1.a) * ccw(&l2.a, &l2.b, &l1.b) <= 0
}

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
    let n = next::<usize>();
    let v = (0..n).map(|_| Point { x: next(), y: next() }).collect::<Vec<_>>();
    let mut p = (1..n).collect::<Vec<_>>();
    let mut ans = 1e20;
    loop {
        p.push(0);
        let mut flag = true;
        'x: for i in 0..n-2 { for j in i+2..n {
            if i == 0 && j == n-1 { continue; }
            if intersect(&Line { a: v[p[i]], b: v[p[i+1]] }, &Line { a: v[p[j]], b: v[p[(j+1)%n]] }) {
                flag = false; break 'x;
            }
        }}

        if flag {
            let mut sum = 0.0;
            for i in 0..n-1 { sum += dist(&v[p[i]], &v[p[i+1]]); }
            sum += dist(&v[p[0]], &v[p[n-1]]);
            if ans > sum { ans = sum; }
        }

        p.pop();
        if !next_permutation(&mut p) { break; }
    }
    if ans == 1e20 { println!("-1"); } else { println!("{:.10}", ans); }
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