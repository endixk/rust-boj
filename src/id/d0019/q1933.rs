// BOJ 1933 [Skyline]
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn ptr(p: &mut *const u8) -> u32 { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as u32 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
pub fn main() {
    let mut p = input(3300033);
    let n = ptr(&mut p) as usize;
    let mut v = vec![];
    for _ in 0..n {
        let (l, x, r) = (ptr(&mut p), ptr(&mut p), ptr(&mut p));
        v.push((l, true, x)); v.push((r, false, x));
    }
    v.sort_unstable();

    let (mut loc, mut sky) = (vec![0], vec![0]);
    let (mut pk, mut it) = (0, 0);
    let mut map = std::collections::BTreeMap::new();
    for (k, t, x) in v {
        if k != pk { loc.push(k); sky.push(0); pk = k; it += 1; }
        if t {
            if map.contains_key(&x) { *map.get_mut(&x).unwrap() += 1; }
            else { map.insert(x, 1); }
        } else {
            if *map.get(&x).unwrap() == 1 { map.remove(&x); }
            else { *map.get_mut(&x).unwrap() -= 1; }
        }
        sky[it] = map.keys().next_back().map_or(0, |&x| x);
    }

    let mut p = 0;
    for (k, x) in loc.into_iter().zip(sky.into_iter()) {
        if p != x { println!("{} {}", k, x); p = x; }
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}