// BOJ 3763 [Sudoku]
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
    id: u32,
    sz: u32,
    h: u32, u: u32, d: u32, l: u32, r: u32
}
#[inline] fn unlink_hor(dl: &mut Vec<Node>, id: usize) {
    let (l, r) = (dl[id].l, dl[id].r);
    dl[l as usize].r = r; dl[r as usize].l = l;
}
#[inline] fn unlink_ver(dl: &mut Vec<Node>, id: usize) {
    let (u, d, h) = (dl[id].u, dl[id].d, dl[id].h);
    dl[u as usize].d = d; dl[d as usize].u = u; dl[h as usize].sz -= 1;
}
#[inline] fn relink_hor(dl: &mut Vec<Node>, id: usize) {
    let (l, r) = (dl[id].l, dl[id].r);
    dl[l as usize].r = id as u32; dl[r as usize].l = id as u32;
}
#[inline] fn relink_ver(dl: &mut Vec<Node>, id: usize) {
    let (u, d, h) = (dl[id].u, dl[id].d, dl[id].h);
    dl[u as usize].d = id as u32; dl[d as usize].u = id as u32; dl[h as usize].sz += 1;
}
fn dlx_build(mat: Vec<Vec<bool>>, n: usize, m: usize) -> Vec<Node> {
    let mut dl = vec![Node::default(); n*m+m+1];

    // link header row
    dl[0].l = m as u32; dl[0].r = 1;
    for j in 1..m {
        dl[j].l = j as u32-1;
        dl[j].r = j as u32+1;
    }
    dl[m].l = m as u32-1; dl[m].r = 0;

    // link rows
    for i in 1..=n {
        dl[i*m+1].l = (i as u32+1)*m as u32; dl[i*m+1].r = i as u32*m as u32+2;
        for j in 2..m {
            dl[i*m+j].l = i as u32*m as u32+j as u32-1;
            dl[i*m+j].r = i as u32*m as u32+j as u32+1;
        }
        dl[(i+1)*m].l = (i as u32+1)*m as u32-1; dl[(i+1)*m].r = i as u32*m as u32+1;
    }

    // link columns
    for j in 1..=m {
        dl[j].u = j as u32+n as u32*m as u32; dl[j].d = j as u32+m as u32; dl[j].sz = n as u32;
        for i in 1..n {
            dl[i*m+j].h = j as u32;
            dl[i*m+j].u = (i as u32-1)*m as u32+j as u32;
            dl[i*m+j].d = (i as u32+1)*m as u32+j as u32;
        }
        dl[n*m+j].u = (n as u32-1)*m as u32+j as u32; dl[n*m+j].d = j as u32; dl[n*m+j].h = j as u32;
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
    while i != h as u32 {
        let mut j = dl[i as usize].r;
        while j != i {
            unlink_ver(dl, j as usize);
            j = dl[j as usize].r;
        }
        i = dl[i as usize].d;
    }
}
fn dlx_uncover(dl: &mut Vec<Node>, h: usize) {
    let mut i = dl[h].u;
    while i != h as u32 {
        let mut j = dl[i as usize].l;
        while j != i {
            relink_ver(dl, j as usize);
            j = dl[j as usize].l;
        }
        i = dl[i as usize].u;
    }
    relink_hor(dl, h);
}
fn dlx(dl: &mut Vec<Node>, sol: &mut Vec<usize>) -> bool {
    if dl[0].r == 0 { return true; }

    // find column with minimum size
    let mut h = dl[0].r;
    let (mut mh, mut ms) = (h, dl[h as usize].sz);
    while h != 0 {
        if dl[h as usize].sz == 0 { return false; }
        if dl[h as usize].sz < ms {
            mh = h; ms = dl[h as usize].sz;
        }
        h = dl[h as usize].r;
    }

    // cover
    dlx_cover(dl, mh as usize);
    let mut i = dl[mh as usize].d;
    while i != mh {
        // push solution
        sol.push(i as usize);
        // cover columns
        let mut j = dl[i as usize].r;
        while j != i {
            dlx_cover(dl, dl[j as usize].h as usize);
            j = dl[j as usize].r;
        }
        // recursion
        if dlx(dl, sol) { return true; }
        else { // backtrack
            sol.pop();
            let mut j = dl[i as usize].l;
            while j != i {
                dlx_uncover(dl, dl[j as usize].h as usize);
                j = dl[j as usize].l;
            }
        }
        i = dl[i as usize].d;
    }

    dlx_uncover(dl, mh as usize);
    false
}

#[inline] fn matr(r: usize, c: usize, x: usize) -> usize { (r-1)*256 + (c-1)*16 + x-1 }
#[inline] fn matc(r: usize, c: usize, x: usize, t: u8) -> usize {
    return match t {
        1 => { 256 + (r-1)*16 + x-1 }, // row
        2 => { 512 + (c-1)*16 + x-1 }, // column
        3 => { 768 + ((r-1)/4*4 + (c-1)/4)*16 + x-1 }, // block
        _ => { (r-1)*16 + c-1 }, // cell
    }
}
#[inline] fn get(id: usize) -> (usize, usize, usize) {
    let id = (id-1)/1024-1;
    (id/256+1, id%256/16+1, id%16+1)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut mat = vec![vec![false; 1024]; 4096];
    for r in 1..17 {
        let s = next::<String>(&mut it);
        for (c, k) in s.chars().enumerate() {
            let (c, x) = (c+1, if k == '-' { 0 } else { k as usize - 'A' as usize + 1 });
            if x > 0 {
                for t in 0..4 { mat[matr(r, c, x)][matc(r, c, x, t)] = true; }
            } else {
                for x in 1..17 { for t in 0..4 {
                    mat[matr(r, c, x)][matc(r, c, x, t)] = true;
                }}
            }
        }
    }

    let mut dl = dlx_build(mat, 4096, 1024);
    let mut sol = Vec::new();
    dlx(&mut dl, &mut sol);

    let mut ans = vec![vec![0; 16]; 16];
    for id in sol {
        let (r, c, x) = get(id);
        ans[r-1][c-1] = x-1;
    }
    for r in 0..16 {
        for c in 0..16 {
            write!(so, "{}", (ans[r][c] as u8 + 'A' as u8) as char).unwrap();
        }
        writeln!(so).unwrap();
    }
}
