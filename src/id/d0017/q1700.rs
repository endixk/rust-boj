// BOJ 1700 [Power Strip Scheduling]
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

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let v = (0..k).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut nxt = vec![k; k];
    for i in 0..k {
        for j in i + 1..k {
            if v[i] == v[j] {
                nxt[i] = j;
                break;
            }
        }
    }

    let mut cnt = 0;
    let mut plug = vec![(0, 0); n];
    for (i, &x) in v.iter().enumerate() {
        let mut ex = n;
        for (j, &(p, _)) in plug.iter().enumerate() {
            if p == x {
                ex = j;
                break;
            }
        }
        if ex < n {
            plug[ex].1 = nxt[i];
            continue;
        }

        let (mut nx, mut mx) = (0, 0);
        for (j, &(_, t)) in plug.iter().enumerate() {
            if t == 0 {
                nx = j;
                mx = 0;
                break;
            } else if t > mx {
                mx = t;
                nx = j;
            }
        }

        plug[nx] = (x, nxt[i]);
        if mx > 0 { cnt += 1; }
    }

    println!("{}", cnt);
}
