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

#[derive(Copy, Clone)]
struct Segment {
    a: i64,
    b: i64,
    s: f64,
}
fn sect(p: &Segment, q: &Segment) -> f64 {
    (q.b - p.b) as f64 / (p.a - q.a) as f64
}
fn find(sv: &[f64], x: f64) -> usize {
    sv.binary_search_by(|&y| y.partial_cmp(&x).unwrap()).unwrap_err() - 1
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut v = (0..n).map(|_| (next::<i64>(&mut it), next::<i64>(&mut it))).collect::<Vec<_>>();
    v.sort_unstable();
    let mut w = vec![];
    for (a, b) in v {
        if let Some((c, _)) = w.last() {
            if a == *c { w.pop(); }
        }
        w.push((a, b));
    }

    let mut cht = vec![];
    cht.push(Segment { a: w[0].0, b: w[0].1, s: 0.0 });
    for i in 1..w.len() {
        let mut nseg = Segment { a: w[i].0, b: w[i].1, s: 0.0 };
        while let Some(&p) = cht.last() {
            nseg.s = sect(&p, &nseg);
            if p.s < nseg.s {
                break;
            }
            cht.pop();
        }
        cht.push(nseg);
    }
    let sv = cht.iter().map(|s| s.s).collect::<Vec<_>>();

    'q: for _ in 0..q {
        let (mut x, y) = (next::<i64>(&mut it), next::<i64>(&mut it));
        let mut cnt = 0;
        while x < y {
            let i = find(&sv, x as f64 + 1e-9);
            if cht[i].a == 1 {
                if cht[i].b <= 0 {
                    writeln!(so, "-1").ok();
                    continue 'q;
                }
                if i == cht.len() - 1 {
                    cnt += (y - x) / cht[i].b + if (y - x) % cht[i].b == 0 { 0 } else { 1 };
                    x = y;
                } else {
                    let dx = cht[i+1].s.ceil() as i64 - x;
                    cnt += dx / cht[i].b + if dx % cht[i].b == 0 { 0 } else { 1 };
                    x += cht[i].b * cnt;
                }
            } else {
                let nx = cht[i].a * x + cht[i].b;
                if nx <= x {
                    writeln!(so, "-1").ok();
                    continue 'q;
                }
                x = nx;
                cnt += 1;
            }
        }
        writeln!(so, "{}", cnt).ok();
    }
}