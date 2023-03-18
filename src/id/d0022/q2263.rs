// BOJ 2263 [Traversals of a Binary Tree]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = it.by_ref().next().unwrap().parse().unwrap();

    let iord = it.by_ref().take(n).map(|x| x.parse().unwrap()).collect::<Vec<u32>>();
    let pord = it.by_ref().take(n).map(|x| x.parse().unwrap()).collect::<Vec<u32>>();
    let mut iidx = vec![0; n+1];
    for i in 0..n { iidx[iord[i] as usize] = i; }

    let mut stk: Vec<(isize, isize, isize, isize)> = Vec::new();
    let mut pre = Vec::new();
    stk.push((0, n as isize -1, 0, n as isize -1));
    while let Some((il, ir, pl, pr)) = stk.pop() {
        if il > ir { continue; }
        let root = pord[pr as usize];
        pre.push(root);
        let idx = iidx[root as usize] as isize;
        stk.push((idx+1, ir, pr-(ir-idx), pr-1));
        stk.push((il, idx-1, pl, pr-(ir-idx)-1));
    }

    for i in 0..n {
        write!(so, "{} ", pre[i]).unwrap();
    }
}
