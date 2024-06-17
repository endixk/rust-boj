// BOJ 8894 [Pattern Lock]
use std::collections::HashMap;
fn mid(a: usize, b: usize) -> usize {
    match a {
        1 => match b { 3 => 2, 7 => 4, 9 => 5, _ => 0 },
        2 => match b { 8 => 5, _ => 0 },
        3 => match b { 1 => 2, 7 => 5, 9 => 6, _ => 0 },
        4 => match b { 6 => 5, _ => 0 },
        6 => match b { 4 => 5, _ => 0 },
        7 => match b { 1 => 4, 3 => 5, 9 => 8, _ => 0 },
        8 => match b { 2 => 5, _ => 0 },
        9 => match b { 1 => 5, 3 => 6, 7 => 8, _ => 0 },
        _ => 0
    }
}
fn track(c: usize, ebit: usize, emap: &HashMap<(usize, usize), usize>,
         path: usize, pc: u8, pv: &mut Vec<bool>, pmap: &mut HashMap<usize, usize>) {
    pmap.insert(ebit, path);
    if pc == 9 { return; }
    for i in 1..10 {
        if pv[i] { continue; }
        match mid(c, i) {
            0 => {
                pv[i] = true;
                track(i, ebit | 1<<emap[&(c, i)], emap, path * 10 + i, pc + 1, pv, pmap);
                pv[i] = false;
            },
            m => {
                if !pv[m] { continue; }
                pv[i] = true;
                track(i, ebit | 1<<emap[&(c, m)] | 1<<emap[&(m, i)], emap, path * 10 + i, pc + 1, pv, pmap);
                pv[i] = false;
            }
        }
    }
}
pub fn main() { read();
    let mut emap = HashMap::<(usize, usize), usize>::new();
    let mut idx = 1;
    for i in 1..10 { for j in i+1..10 {
        if mid(i, j) == 0 {
            emap.insert((i, j), idx);
            emap.insert((j, i), idx);
            idx += 1;
        }
    }}

    let mut pmap = HashMap::<usize, usize>::new();
    for i in 1..10 {
        let mut pv = vec![false; 10];
        pv[i] = true;
        track(i, 0, &emap, i, 1, &mut pv, &mut pmap);
    }

    for _ in 0..next() {
        let mut ebit = 0;
        for _ in 0..next() {
            let (a, b) = (next::<usize>(), next::<usize>());
            match mid(a, b) {
                0 => ebit |= 1<<emap[&(a, b)],
                m => ebit |= 1<<emap[&(a, m)] | 1<<emap[&(m, b)]
            }
        }

        if let Some(v) = pmap.get(&ebit) {
            println!("{}", v.to_string().chars().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
        } else {
            println!("IMPOSSIBLE");
        }
    }
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