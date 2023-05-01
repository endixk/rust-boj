// BOJ 8131 [Ploughing]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn query(ps: &Vec<i64>, i1: usize, i2: usize, j1: usize, j2: usize, m: usize) -> i64 {
    ps[(i2+1)*(m+1)+(j2+1)] - ps[(i2+1)*(m+1)+j1] - ps[i1*(m+1)+(j2+1)] + ps[i1*(m+1)+j1]
}
fn simulate(ps: &Vec<i64>, lb: usize, rb: usize, n: usize, m: usize, k: i64) -> bool {
    // try to plough the field greedily,
    // with restricted numbers of vertical ploughs from left and right
    let (mut u, mut d, mut l, mut r) = (0, n-1, 0, m-1);
    while u <= d {
        // left column is ploughable
        if l < lb && query(ps, u, d, l, l, m) <= k { l += 1; continue }
        // right column is ploughable
        if r > rb && query(ps, u, d, r, r, m) <= k { r -= 1; continue }
        // upper row is ploughable
        if query(ps, u, u, l, r, m) <= k { u += 1; continue }
        // lower row is ploughable
        if query(ps, d, d, l, r, m) <= k { d -= 1; continue }
        // cannot plough
        return false;
    }
    true
}

fn solve(field: &Vec<i32>, n: usize, m: usize, k: i64) -> usize {
    // precompute prefix sum
    let mut ps = vec![0; (n+1)*(m+1)];
    for i in 0..n { for j in 0..m {
        ps[(i+1)*(m+1)+(j+1)] = field[i*m+j] as i64 + ps[i*(m+1)+(j+1)] + ps[(i+1)*(m+1)+j] - ps[i*(m+1)+j];
    }}

    // precompute ploughable area
    let mut plo = vec![0; n*(m+1)];
    for i in 0..n {
        // two-pointer
        let (mut lb, mut rb) = (0, 0);
        while lb <= m {
            while rb < m && query(&ps, i, i, lb, rb, m) <= k {
                rb += 1;
            }
            plo[i*(m+1)+lb] = rb-1;
            lb += 1;
        }
    }

    let mut ans = n+m-1;
    // fix number of vertical ploughs from the left
    for lb in 0..=m {
        // find the maximum number of vertical ploughs from the right
        let mut rb = m-1;
        (0..n).for_each(|i| {
            rb = rb.min(plo[i*(m+1)+lb]);
        });
        // simulate ploughing
        if simulate(&ps, lb, rb, n, m, k) {
            ans = ans.min(lb + m-1-rb + n);
        }
    }

    ans
}

fn rotate(field: Vec<i32>, n: usize, m: usize) -> Vec<i32> {
    let mut field_t = vec![0; n*m];
    for i in 0..n { for j in 0..m {
        field_t[j*n+i] = field[i*m+j];
    }}
    field_t
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    // read input
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (k, m, n) = (next::<i64>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut field = vec![0; m*n];
    for i in 0..n { for j in 0..m {
        field[i*m+j] = next::<i32>(&mut it);
    }}

    let ans = solve(&field, n, m, k);
    let field = rotate(field, n, m);
    writeln!(so, "{}", ans.min(solve(&field, m, n, k))).ok();
}
