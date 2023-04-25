// BOJ 12094 [2048 (Hard)]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn move_l(board: &[[i32; 20]; 20], size: usize) -> [[i32; 20]; 20] {
    let mut next = [[0; 20]; 20];

    for row in 0..size {
        // collect
        let mut v = [0; 20];
        let mut vlen = 0;
        for col in 0..size {
            if board[row][col] != 0 {
                v[vlen] = board[row][col];
                vlen += 1;
            }
        }

        // merge
        let mut nv = [0; 20];
        let mut nvlen = 0;
        let mut i = 0;
        while i < vlen {
            if i + 1 < vlen && v[i] == v[i + 1] {
                nv[nvlen] = v[i] * 2;
                nvlen += 1;
                i += 2;
            } else {
                nv[nvlen] = v[i];
                nvlen += 1;
                i += 1;
            }
        }

        // fill
        for col in 0..nvlen {
            next[row][col] = nv[col];
        }
    }

    next
}
fn move_r(board: &[[i32; 20]; 20], size: usize) -> [[i32; 20]; 20] {
    let mut next = [[0; 20]; 20];

    for row in 0..size {
        // collect
        let mut v = [0; 20];
        let mut vlen = 0;
        for col in (0..size).rev() {
            if board[row][col] != 0 {
                v[vlen] = board[row][col];
                vlen += 1;
            }
        }

        // merge
        let mut nv = [0; 20];
        let mut nvlen = 0;
        let mut i = 0;
        while i < vlen {
            if i + 1 < vlen && v[i] == v[i + 1] {
                nv[nvlen] = v[i] * 2;
                nvlen += 1;
                i += 2;
            } else {
                nv[nvlen] = v[i];
                nvlen += 1;
                i += 1;
            }
        }

        // fill
        for col in 0..nvlen {
            next[row][size - 1 - col] = nv[col];
        }
    }

    next
}
fn move_u(board: &[[i32; 20]; 20], size: usize) -> [[i32; 20]; 20] {
    let mut next = [[0; 20]; 20];

    for col in 0..size {
        // collect
        let mut v = [0; 20];
        let mut vlen = 0;
        for row in 0..size {
            if board[row][col] != 0 {
                v[vlen] = board[row][col];
                vlen += 1;
            }
        }

        // merge
        let mut nv = [0; 20];
        let mut nvlen = 0;
        let mut i = 0;
        while i < vlen {
            if i + 1 < vlen && v[i] == v[i + 1] {
                nv[nvlen] = v[i] * 2;
                nvlen += 1;
                i += 2;
            } else {
                nv[nvlen] = v[i];
                nvlen += 1;
                i += 1;
            }
        }

        // fill
        for row in 0..nvlen {
            next[row][col] = nv[row];
        }
    }

    next
}
fn move_d(board: &[[i32; 20]; 20], size: usize) -> [[i32; 20]; 20] {
    let mut next = [[0; 20]; 20];

    for col in 0..size {
        // collect
        let mut v = [0; 20];
        let mut vlen = 0;
        for row in (0..size).rev() {
            if board[row][col] != 0 {
                v[vlen] = board[row][col];
                vlen += 1;
            }
        }

        // merge
        let mut nv = [0; 20];
        let mut nvlen = 0;
        let mut i = 0;
        while i < vlen {
            if i + 1 < vlen && v[i] == v[i + 1] {
                nv[nvlen] = v[i] * 2;
                nvlen += 1;
                i += 2;
            } else {
                nv[nvlen] = v[i];
                nvlen += 1;
                i += 1;
            }
        }

        // fill
        for row in 0..nvlen {
            next[size - 1 - row][col] = nv[row];
        }
    }

    next
}

static mut MAX: i32 = 0;
fn eq(bx: &[[i32; 20]; 20], by: &[[i32; 20]; 20], size: usize) -> bool {
    for i in 0..size {
        for j in 0..size {
            if bx[i][j] != by[i][j] {
                return false;
            }
        }
    }
    true
}
fn dfs(board: &[[i32; 20]; 20], size: usize, depth: usize, max_depth: usize) -> i32 {
    let mut ret = *board.iter().map(|row| row.iter().max().unwrap()).max().unwrap();
    if depth == max_depth { unsafe {MAX = MAX.max(ret);} return ret; }
    if (ret << (max_depth - depth)) <= unsafe {MAX} { return 0; }

    let next = move_l(board, size);
    if !eq(board, &next, size) {
        ret = ret.max(dfs(&next, size, depth + 1, max_depth));
    }
    let next = move_r(board, size);
    if !eq(board, &next, size) {
        ret = ret.max(dfs(&next, size, depth + 1, max_depth));
    }
    let next = move_u(board, size);
    if !eq(board, &next, size) {
        ret = ret.max(dfs(&next, size, depth + 1, max_depth));
    }
    let next = move_d(board, size);
    if !eq(board, &next, size) {
        ret = ret.max(dfs(&next, size, depth + 1, max_depth));
    }

    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = next(&mut it);
    let mut a = [[0; 20]; 20];
    (0..n).for_each(|i| (0..n).for_each(|j| a[i][j] = next(&mut it)));
    writeln!(so, "{}", dfs(&a, n, 0, 10)).unwrap();
}
