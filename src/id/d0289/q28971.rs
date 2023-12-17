// BOJ 28971 [Two Maps]
// Supported by GitHub Copilot

use std::collections::{HashMap, BTreeSet};
fn overlap(lmap: &HashMap<i64, usize>, lset: &Vec<BTreeSet<(i64, usize)>>,
           rmap: &HashMap<i64, usize>, rset: &Vec<BTreeSet<(i64, usize)>>,
           smap: &HashMap<i64, usize>, sset: &Vec<BTreeSet<(i64, usize)>>,
           l: i64, r: i64, s: i64) -> usize {
    let mut ans = 0;
    if r - l < s {
        if lmap.contains_key(&(r - s)) {
            let set = &lset[lmap[&(r - s)]];
            ans += set.range((l, 0)..(r + 1, 0)).count();
        }
        if rmap.contains_key(&(l + s)) {
            let set = &rset[rmap[&(l + s)]];
            ans += set.range((l, 0)..(r + 1, 0)).count();
        }
        if smap.contains_key(&(s + l - r)) {
            let set = &sset[smap[&(s + l - r)]];
            ans += set.range(..(r - s, 0)).count();
            ans += set.range((r + 1, 0)..).count();
        }
        if smap.contains_key(&s) {
            let set = &sset[smap[&s]];
            ans += set.range((r - s + 1, 0)..(l, 0)).count();
        }
    } else {
        for &key in smap.keys() {
            let set = &sset[smap[&key]];
            ans += set.range((l, 0)..(r - key + 1, 0)).count();
        }
    }

    ans
}
pub fn main() { read();
    let (s, n) = (next::<i64>(), next::<usize>());

    let mut lc = 0;
    let mut lmap = HashMap::<i64, usize>::new();
    let mut lset = Vec::<BTreeSet<(i64, usize)>>::new();
    let mut rc = 0;
    let mut rmap = HashMap::<i64, usize>::new();
    let mut rset = Vec::<BTreeSet<(i64, usize)>>::new();
    let mut sc = 0;
    let mut smap = HashMap::<i64, usize>::new();
    let mut sset = Vec::<BTreeSet<(i64, usize)>>::new();

    let mut q = vec![];
    let mut ans = 0;
    for i in 0..n {
        let k = next::<u8>();
        if k == 1 {
            let (l, r) = (next::<i64>(), next::<i64>());
            q.push((l, r));
            if r - l > s { println!("{}", ans); continue; }
            ans += overlap(&lmap, &lset, &rmap, &rset, &smap, &sset, l, r, s);

            if !lmap.contains_key(&l) {
                lmap.insert(l, lc);
                lset.push(BTreeSet::new());
                lc += 1;
            }
            lset[lmap[&l]].insert((r, i));

            if !rmap.contains_key(&r) {
                rmap.insert(r, rc);
                rset.push(BTreeSet::new());
                rc += 1;
            }
            rset[rmap[&r]].insert((l, i));

            if !smap.contains_key(&(r - l)) {
                smap.insert(r - l, sc);
                sset.push(BTreeSet::new());
                sc += 1;
            }
            sset[smap[&(r - l)]].insert((l, i));
        } else {
            let k = next::<usize>() - 1;
            let (l, r) = q[k];
            q.push((0, 0));
            if r - l > s { println!("{}", ans); continue; }
            lset[lmap[&l]].remove(&(r, k));
            rset[rmap[&r]].remove(&(l, k));
            sset[smap[&(r - l)]].remove(&(l, k));
            ans -= overlap(&lmap, &lset, &rmap, &rset, &smap, &sset, l, r, s);
        }
        println!("{}", ans);
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