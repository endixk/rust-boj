// BOJ 14865 [Cutting Curves]
// Supported by GitHub Copilot

use std::io::{self, Read};

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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| (next::<i64>(&mut it), next::<i64>(&mut it))).collect::<Vec<_>>();

    let (mut mx, mut mi) = (1<<31, 0);
    for i in 0..n {
        let (vs, ve) = (v[i], v[(i + 1) % n]);
        if vs.0 == ve.0 && vs.1 * ve.1 < 0 && vs.0 < mx {
            mx = vs.0;
            mi = i;
        }
    }

    let mut hills = vec![];
    let mut h = 0;
    let mut cnt = 0;
    loop {
        while v[mi].0 != v[(mi + 1) % n].0 || v[mi].1 * v[(mi + 1) % n].1 >= 0 {
            mi = (mi + 1) % n; cnt += 1;
        }
        if cnt >= n { break; }
        let s = v[mi].0;

        mi = (mi + 1) % n; cnt += 1;
        while v[mi].0 != v[(mi + 1) % n].0 || v[mi].1 * v[(mi + 1) % n].1 >= 0 {
            mi = (mi + 1) % n; cnt += 1;
        }
        let e = v[mi].0;

        hills.push((s, h));
        hills.push((e, h));
        mi = (mi + 1) % n; cnt += 1;
        h += 1;
    }
    hills.sort_unstable();

    let (mut x, mut y) = (0, 0);
    let mut st = vec![];
    let mut i = 0;
    while i < hills.len() {
        if i+1 < hills.len() && hills[i].1 == hills[i+1].1 {
            y += 1;
            x += if st.is_empty() { 1 } else { 0 };
            i += 2;
            continue;
        }
        else if st.is_empty() {
            st.push(hills[i].1); x += 1;
        } else {
            if st[st.len()-1] == hills[i].1 {
                st.pop();
            } else {
                st.push(hills[i].1);
            }
        }
        i += 1;
    }

    println!("{} {}", x, y);
}
