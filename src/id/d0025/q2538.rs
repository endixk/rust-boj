// BOJ 2538 [Graph Paper Cutting]
fn corner(x: i64, y: i64, w: i64, h: i64) -> bool {
    return x == 0 && y == 0 || x == w && y == 0 || x == w && y == h || x == 0 && y == h;
}
fn eid(x: i64, y: i64, w: i64, h: i64) -> i64 {
    return if y == 0 { 1 } else if x == w { 2 } else if y == h { 3 } else if x == 0 { 4 } else { 0 };
}
fn dist(x1: i64, y1: i64, x2: i64, y2: i64, w: i64, h: i64) -> i64 {
    let k1 = eid(x1, y1, w, h); assert_ne!(k1, 0);
    let k2 = eid(x2, y2, w, h); assert_ne!(k2, 0);
    if k1 == k2 {
        return match k1 {
            1 if x1 < x2 => x2 - x1,
            1 => 2 * w + 2 * h - (x1 - x2),
            2 if y1 < y2 => y2 - y1,
            2 => 2 * w + 2 * h - (y1 - y2),
            3 if x1 > x2 => x1 - x2,
            3 => 2 * w + 2 * h - (x2 - x1),
            4 if y1 > y2 => y1 - y2,
            _ => 2 * w + 2 * h - (y2 - y1)
        }
    }

    let d1 = match k1 { // distance from (x1, y1) to the next corner
        1 => w - x1, 2 => h - y1, 3 => x1, _ => y1
    };
    let d2 = match k2 { // distance from (x2, y2) to the previous corner
        1 => x2, 2 => y2, 3 => w - x2, _ => h - y2
    };
    let mut ret = d1;
    for k in k1+1.. {
        let k = (k - 1) % 4 + 1;
        if k == k2 { break; }
        else if k == 1 || k == 3 { ret += w; }
        else { ret += h; }
    }
    ret + d2
}
fn ldist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    return if x1 == x2 { (y1 - y2).abs() } else if y1 == y2 { (x1 - x2).abs() } else { panic!() };
}
pub fn main() { read();
    let (w, h) = (next::<i64>(), next::<i64>());
    let n = next::<usize>();
    let p = (0..n).map(|_| (next::<i64>(), next::<i64>())).collect::<Vec<_>>();

    // find first split
    let mut t = n;
    let mut sum = 0;
    let mut cnr = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        if corner(p[i].0, p[i].1, w, h) { cnr += 1; continue; }
        if corner(p[j].0, p[j].1, w, h) { cnr += 1; continue; }
        if eid(p[i].0, p[i].1, w, h) != 0 && eid(p[i].0, p[i].1, w, h) != eid(p[j].0, p[j].1, w, h) {
            t = i;
            break;
        }
        sum += ldist(p[i].0, p[i].1, p[j].0, p[j].1);
    }
    if p.len() == 4 && cnr == 4 { println!("0 0"); return; }
    if t == n { println!("1 {}", sum + 2 * w + 2 * h); return; }

    let (mut cnt, mut ans) = (0, 0);
    let mut prv = t;
    loop {
        let mut len = 0;
        let mut nxt = 0;
        for i in prv.. {
            let (i, j) = (i % n, (i + 1) % n);
            len += ldist(p[i].0, p[i].1, p[j].0, p[j].1);
            if eid(p[j].0, p[j].1, w, h) != 0 {
                nxt = j;
                break;
            }
        }

        cnt += 1;
        ans = ans.max(len + dist(p[prv].0, p[prv].1, p[nxt].0, p[nxt].1, w, h));

        prv = nxt;
        loop {
            let nxt = (prv + 1) % n;
            if corner(p[prv].0, p[prv].1, w, h) { prv = nxt; continue; }
            if corner(p[nxt].0, p[nxt].1, w, h) { prv = nxt; continue; }
            if eid(p[prv].0, p[prv].1, w, h) != eid(p[nxt].0, p[nxt].1, w, h) { break; }
            prv = nxt;
        }
        if prv == t { break; }
    }

    println!("{} {}", cnt, ans);
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