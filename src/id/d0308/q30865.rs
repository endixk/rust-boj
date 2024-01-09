// BOJ 30865 [XOR and Queries]
// Supported by GitHub Copilot

struct Node {
    cnt: u32,
    nxt: (usize, usize),
}
fn insert(trie: &mut Vec<Node>, a: &[bool], i: usize, cur: usize, it: &mut usize) {
    trie[cur].cnt += 1;
    if i == a.len() { return; }
    if a[i] && trie[cur].nxt.1 == 0 {
        trie[cur].nxt.1 = *it; *it += 1;
        trie.push(Node { cnt: 0, nxt: (0, 0) });
    }
    if !a[i] && trie[cur].nxt.0 == 0 {
        trie[cur].nxt.0 = *it; *it += 1;
        trie.push(Node { cnt: 0, nxt: (0, 0) });
    }
    insert(trie, a, i + 1, if a[i] { trie[cur].nxt.1 } else { trie[cur].nxt.0 }, it);
}
fn delete(trie: &mut Vec<Node>, a: &[bool], i: usize, cur: usize) {
    trie[cur].cnt -= 1;
    if i == a.len() { return; }
    delete(trie, a, i + 1, if a[i] { trie[cur].nxt.1 } else { trie[cur].nxt.0 });
}
fn kth(trie: &Vec<Node>, cur: usize, a: &[bool], i: usize, k: u32, ret: &mut Vec<bool>) {
    if i == a.len() { return; }
    if trie[cur].nxt.0 == 0 {
        ret.push(true);
        kth(trie, trie[cur].nxt.1, a, i + 1, k, ret);
    } else if trie[cur].nxt.1 == 0 {
        ret.push(false);
        kth(trie, trie[cur].nxt.0, a, i + 1, k, ret);
    } else {
        let (l, r) = (trie[cur].nxt.0, trie[cur].nxt.1);
        if a[i] {
            if trie[l].cnt >= k {
                ret.push(false);
                kth(trie, l, a, i + 1, k, ret);
            } else {
                ret.push(true);
                kth(trie, r, a, i + 1, k - trie[l].cnt, ret);
            }
        } else {
            if trie[r].cnt >= k {
                ret.push(true);
                kth(trie, r, a, i + 1, k, ret);
            } else {
                ret.push(false);
                kth(trie, l, a, i + 1, k - trie[r].cnt, ret);
            }
        }
    }
}
#[inline] fn xtoa(x: u32) -> Vec<bool> {
    let mut ret = vec![false; 32];
    for i in 0..32 { ret[31-i] = x & (1 << i) != 0; }
    ret
}
#[inline] fn atox(a: &[bool]) -> u32 {
    let mut ret = 0;
    for i in 0..32 { if a[i] { ret |= 1 << (31 - i); } }
    ret
}
pub fn main() { read();
    let n = next::<usize>();
    let mut arr = vec![0; n];
    let mut trie = vec![Node { cnt: 0, nxt: (0, 0) }];
    let mut it = 1;
    for i in 0..n {
        let x = next::<u32>();
        let a = xtoa(x);
        arr[i] = x;
        insert(&mut trie, &a, 0, 0, &mut it);
    }

    for _ in 0..next() {
        let (q, i, x) = (next::<u8>(), next::<usize>(), next::<u32>());
        if q == 1 {
            delete(&mut trie, &xtoa(arr[i-1]), 0, 0);
            insert(&mut trie, &xtoa(x), 0, 0, &mut it);
            arr[i-1] = x;
        } else {
            let mut ret = Vec::new();
            kth(&trie, 0, &xtoa(x), 0, i as u32, &mut ret);
            println!("{}", atox(&ret) ^ x);
        }
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