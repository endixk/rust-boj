// BOJ 13546 [Sequence and Queries 4]
use std::collections::VecDeque;
fn add(i: usize, front: bool, arr: &[u32], sq: usize,
       cnt: &mut Vec<u32>, bkt: &mut Vec<u32>, dqv: &mut Vec<VecDeque<u32>>) {
    let x = arr[i] as usize;
    if !dqv[x].is_empty() {
        let d = *dqv[x].back().unwrap() - *dqv[x].front().unwrap();
        cnt[d as usize] -= 1; bkt[d as usize / sq] -= 1;
    }
    if front { dqv[x].push_front(i as u32); }
    else { dqv[x].push_back(i as u32); }
    let d = *dqv[x].back().unwrap() - *dqv[x].front().unwrap();
    cnt[d as usize] += 1; bkt[d as usize / sq] += 1;
}
fn sub(i: usize, front: bool, arr: &[u32], sq: usize,
       cnt: &mut Vec<u32>, bkt: &mut Vec<u32>, dqv: &mut Vec<VecDeque<u32>>) {
    let x = arr[i] as usize;
    let d = *dqv[x].back().unwrap() - *dqv[x].front().unwrap();
    cnt[d as usize] -= 1; bkt[d as usize / sq] -= 1;
    if front { dqv[x].pop_front(); }
    else { dqv[x].pop_back(); }
    if !dqv[x].is_empty() {
        let d = *dqv[x].back().unwrap() - *dqv[x].front().unwrap();
        cnt[d as usize] += 1; bkt[d as usize / sq] += 1;
    }
}
fn qry(cnt: &[u32], bkt: &[u32], sq: usize) -> usize {
    for i in (0..sq).rev().filter(|&i| bkt[i] > 0) {
        if let Some(j) = (0..sq).rfind(|&j| cnt[i * sq + j] > 0) {
            return i * sq + j;
        }
    }
    0
}

pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let sq = (n as f64).sqrt().ceil() as usize;
    assert!(sq * sq >= n);
    let arr = (0..n).map(|_| next::<u32>()).collect::<Vec<_>>();

    let m = next::<usize>();
    let mut qv = (0..m).map(|i|
        (next::<usize>() - 1, next::<usize>() - 1, i)
    ).collect::<Vec<_>>();
    qv.sort_unstable_by(|a, b| (a.0 / sq).cmp(&(b.0 / sq)).then(a.1.cmp(&b.1)));

    let mut cnt = vec![0; sq*sq];
    let mut bkt = vec![0; sq];
    let mut dqv = vec![VecDeque::new(); k+1];
    let mut ans = vec![0; m];
    let (mut l, mut r) = (qv[0].0, qv[0].1);
    for i in l..=r { add(i, false, &arr, sq, &mut cnt, &mut bkt, &mut dqv); }
    ans[qv[0].2] = qry(&cnt, &bkt, sq);
    for (nl, nr, i) in qv.into_iter().skip(1) {
        while l > nl { l -= 1; add(l, true, &arr, sq, &mut cnt, &mut bkt, &mut dqv); }
        while r < nr { r += 1; add(r, false, &arr, sq, &mut cnt, &mut bkt, &mut dqv); }
        while l < nl { sub(l, true, &arr, sq, &mut cnt, &mut bkt, &mut dqv); l += 1; }
        while r > nr { sub(r, false, &arr, sq, &mut cnt, &mut bkt, &mut dqv); r -= 1; }
        ans[i] = qry(&cnt, &bkt, sq);
    }

    for i in 0..m { println!("{}", ans[i]); }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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