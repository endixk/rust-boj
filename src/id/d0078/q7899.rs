// BOJ 7899 [Markov Trains]
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

#[derive(Debug)] struct Train {
    dep: i32, arr: i32, dst: usize, prb: f64,
}
#[derive(Debug)] struct Stop {
    id: usize, sch: Vec<Train>,
}
struct Path {
    cur: usize, t: i32, ord: Vec<usize>, prb: f64, i: usize,
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let mut stops = (0..12).map(|i| Stop { id: i, sch: Vec::new() }).collect::<Vec<_>>();
        for _ in 0..n {
            let (src, dep, dst, arr, prb) = (
                next::<char>(&mut it), next::<String>(&mut it),
                next::<char>(&mut it), next::<String>(&mut it),
                1.0 - next::<f64>(&mut it),
            );
            let (src, dst) = (src as usize - 'A' as usize, dst as usize - 'A' as usize);
            let dep = dep[0..2].parse::<i32>().unwrap() * 60 + dep[3..5].parse::<i32>().unwrap();
            let arr = arr[0..2].parse::<i32>().unwrap() * 60 + arr[3..5].parse::<i32>().unwrap();
            stops[src].sch.push(Train { dep, arr, dst, prb });
        }
        for stop in stops.iter_mut() {
            stop.sch.sort_by_key(|t| t.dep);
        }

        let (src, dep, dst, arr) = (
            next::<char>(&mut it), next::<String>(&mut it),
            next::<char>(&mut it), next::<String>(&mut it),
        );
        let (src, dst) = (src as usize - 'A' as usize, dst as usize - 'A' as usize);
        let dep = dep[0..2].parse::<i32>().unwrap() * 60 + dep[3..5].parse::<i32>().unwrap();
        let arr = arr[0..2].parse::<i32>().unwrap() * 60 + arr[3..5].parse::<i32>().unwrap();

        let mut paths = vec![Path { cur: src, t: dep-1, ord: vec![0; 12], prb: 1.0, i: 1 }];
        paths[0].ord[src] = 1;

        let mut pmap = std::collections::HashMap::new();
        while !paths.is_empty() {
            let mut nxt = vec![];
            for path in &paths {
                if path.cur == dst && path.t <= arr {
                    *pmap.entry(path.ord.clone()).or_insert(0.0) += path.prb;
                    continue;
                }

                let mut pv = vec![1.0; 12];
                for tr in &stops[path.cur].sch {
                    if tr.dep <= path.t { continue; }
                    if path.ord[tr.dst] != 0 { continue; }

                    let mut nord = path.ord.clone();
                    nord[tr.dst] = path.i + 1;
                    nxt.push(Path {
                        cur: tr.dst, t: tr.arr, ord: nord,
                        prb: path.prb * tr.prb * pv[tr.dst], i: path.i + 1,
                    });
                    pv[tr.dst] *= 1.0 - tr.prb;
                }
            }
            paths = nxt;
        }

        let (mut ans, mut ansp) = (0.0, vec![]);
        for (k, v) in pmap {
            if v > ans {
                ans = v;
                ansp = k;
            }
        }
        let mut path = vec!['Z'; 12];
        for (i, &t) in ansp.iter().enumerate() {
            if t != 0 {
                path[t-1] = (i as u8 + 'A' as u8) as char;
            }
        }

        for c in path {
            if c == 'Z' { break; }
            write!(so, "{} ", c)?;
        }
        writeln!(so, "\n{:.4}", ans)?;
    }

    Ok(())
}
