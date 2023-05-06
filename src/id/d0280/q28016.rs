// BOJ 28016 [Prize Drawing]
// Supported by GitHub Copilot
// TODO UNSOLVED

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

fn dfs(balls: &mut Vec<Vec<u128>>, a: &Vec<Vec<u8>>,
       i: usize, j: usize, n: usize, m: usize) -> bool {
    if i == n-1 { return true }
    if a[i+1][j] == 0 {
        balls[i+1][j] += balls[i][j];
        return dfs(balls, a, i+1, j, n, m)
    }

    let mut l = a[i][j-1] == 0 && a[i+1][j-1] == 0;
    let mut r = a[i][j+1] == 0 && a[i+1][j+1] == 0;
    if l {
        balls[i+1][j-1] += balls[i][j] >> 1;
        l = dfs(balls, a, i+1, j-1, n, m);
        if l {
            if r {
                balls[i+1][j+1] += balls[i][j] >> 1;
                r = dfs(balls, a, i+1, j+1, n, m);
            }
            if !r {
                balls[i+1][j-1] += balls[i][j] >> 1;
                dfs(balls, a, i+1, j-1, n, m);
            }
        } else {
            balls[i+1][j+1] += balls[i][j];
            return dfs(balls, a, i+1, j+1, n, m)
        }
    } else {
        return if r {
            balls[i+1][j+1] += balls[i][j];
            dfs(balls, a, i+1, j+1, n, m)
        } else {
            false
        }
    }

    true
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![vec![0; m]; n];
    let mut s = (0, 0);
    for i in 0..n {
        for j in 0..m {
            a[i][j] = next::<u8>(&mut it);
            if a[i][j] == 2 {
                s = (i, j);
                a[i][j] = 0;
            }
        }
    }

    let mut balls = vec![vec![0u128; m]; n];
    balls[s.0][s.1] = 1<<(n+2);
    if !dfs(&mut balls, &a, s.0, s.1, n, m) {
        println!("-1");
        return
    }

    let (mut ans, mut ansi) = (0, 0);
    for i in 0..m {
        if balls[n-1][i] > ans {
            ans = balls[n-1][i];
            ansi = i;
        }
    }
    println!("{}", ansi);
}
