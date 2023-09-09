// BOJ 19998 [Sudoku (Hard)]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
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

// DLX - Knuth's Algorithm X with Dancing Links
#[derive(Default, Clone)] struct Node {
    id: usize,
    sz: usize,
    h: usize, u: usize, d: usize, l: usize, r: usize
}
#[inline] fn unlink_hor(dl: &mut Vec<Node>, id: usize) {
    let (l, r) = (dl[id].l, dl[id].r);
    dl[l].r = r; dl[r].l = l;
}
#[inline] fn unlink_ver(dl: &mut Vec<Node>, id: usize) {
    let (u, d, h) = (dl[id].u, dl[id].d, dl[id].h);
    dl[u].d = d; dl[d].u = u; dl[h].sz -= 1;
}
#[inline] fn relink_hor(dl: &mut Vec<Node>, id: usize) {
    let (l, r) = (dl[id].l, dl[id].r);
    dl[l].r = id; dl[r].l = id;
}
#[inline] fn relink_ver(dl: &mut Vec<Node>, id: usize) {
    let (u, d, h) = (dl[id].u, dl[id].d, dl[id].h);
    dl[u].d = id; dl[d].u = id; dl[h].sz += 1;
}
fn dlx_build(mat: Vec<Vec<bool>>, n: usize, m: usize) -> Vec<Node> {
    let mut dl = vec![Node::default(); n*m+m+1];

    // link header row
    dl[0].l = m; dl[0].r = 1;
    for j in 1..m {
        dl[j].l = j-1;
        dl[j].r = j+1;
    }
    dl[m].l = m-1; dl[m].r = 0;

    // link rows
    for i in 1..=n {
        dl[i*m+1].l = (i+1)*m; dl[i*m+1].r = i*m+2;
        for j in 2..m {
            dl[i*m+j].l = i*m+j-1;
            dl[i*m+j].r = i*m+j+1;
        }
        dl[(i+1)*m].l = (i+1)*m-1; dl[(i+1)*m].r = i*m+1;
    }

    // link columns
    for j in 1..=m {
        dl[j].u = j+n*m; dl[j].d = j+m; dl[j].sz = n;
        for i in 1..n {
            dl[i*m+j].h = j;
            dl[i*m+j].u = (i-1)*m+j;
            dl[i*m+j].d = (i+1)*m+j;
        }
        dl[n*m+j].u = (n-1)*m+j; dl[n*m+j].d = j; dl[n*m+j].h = j;
    }

    // unlink 0s
    for i in 1..=n {
        for j in 1..=m {
            let id = i*m+j;
            if !mat[i-1][j-1] {
                unlink_hor(&mut dl, id);
                unlink_ver(&mut dl, id);
            }
        }
    }

    dl
}
fn dlx_cover(dl: &mut Vec<Node>, h: usize) {
    unlink_hor(dl, h);
    let mut i = dl[h].d;
    while i != h {
        let mut j = dl[i].r;
        while j != i {
            unlink_ver(dl, j);
            j = dl[j].r;
        }
        i = dl[i].d;
    }
}
fn dlx_uncover(dl: &mut Vec<Node>, h: usize) {
    let mut i = dl[h].u;
    while i != h {
        let mut j = dl[i].l;
        while j != i {
            relink_ver(dl, j);
            j = dl[j].l;
        }
        i = dl[i].u;
    }
    relink_hor(dl, h);
}
fn dlx(dl: &mut Vec<Node>, sol: &mut Vec<usize>) -> bool {
    if dl[0].r == 0 { return true; }

    // find column with minimum size
    let mut h = dl[0].r;
    let (mut mh, mut ms) = (h, dl[h].sz);
    while h != 0 {
        if dl[h].sz == 0 { return false; }
        if dl[h].sz < ms {
            mh = h; ms = dl[h].sz;
        }
        h = dl[h].r;
    }

    // cover
    dlx_cover(dl, mh);
    let mut i = dl[mh].d;
    while i != mh {
        // push solution
        sol.push(i);
        // cover columns
        let mut j = dl[i].r;
        while j != i {
            dlx_cover(dl, dl[j].h);
            j = dl[j].r;
        }
        // recursion
        if dlx(dl, sol) { return true; }
        else { // backtrack
            sol.pop();
            let mut j = dl[i].l;
            while j != i {
                dlx_uncover(dl, dl[j].h);
                j = dl[j].l;
            }
        }
        i = dl[i].d;
    }

    dlx_uncover(dl, mh);
    false
}

#[inline] fn matr(r: usize, c: usize, x: usize) -> usize { (r-1)*81 + (c-1)*9 + x-1 }
#[inline] fn matc(r: usize, c: usize, x: usize, t: u8) -> usize {
    return match t {
        1 => { 81 + (r-1)*9 + x-1 }, // row
        2 => { 162 + (c-1)*9 + x-1 }, // column
        3 => { 243 + ((r-1)/3*3 + (c-1)/3)*9 + x-1 }, // block
        _ => { (r-1)*9 + c-1 }, // cell
    }
}
#[inline] fn get(id: usize) -> (usize, usize, usize) {
    let id = (id-1)/324-1;
    (id/81+1, id%81/9+1, id%9+1)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut b = vec![vec![0; 9]; 9];
    for t in 0..81 {
        let (i, j, x) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        if b[i-1][j-1] > 0 { writeln!(so, "{}", t+1).ok(); return; }
        b[i-1][j-1] = x;

        let mut mat = vec![vec![false; 324]; 729];
        for r in 1..10 {
            for c in 1..10 {
                let x = b[r-1][c-1];
                if x > 0 {
                    for t in 0..4 { mat[matr(r, c, x)][matc(r, c, x, t)] = true; }
                } else {
                    for x in 1..10 {
                        for t in 0..4 {
                            mat[matr(r, c, x)][matc(r, c, x, t)] = true;
                        }
                    }
                }
            }
        }

        let mut dl = dlx_build(mat, 729, 324);
        let mut sol = Vec::new();
        if !dlx(&mut dl, &mut sol) {
            writeln!(so, "{}", t+1).ok();
            return;
        }
    }
    writeln!(so, "-1").ok();
}
