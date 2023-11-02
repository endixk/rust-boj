// BOJ 25211 [Counting Squares]
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

fn case(x1: i64, p1: i64, x2: i64, p2: i64, n: i64) -> i64 {
    let s = n * (n - 1) / 2;
    let sq = n * (n - 1) * (2 * n - 1) / 6;
    p1 * p2 * sq + (x1 * p2 + x2 * p1) * s + x1 * x2 * n
}
fn overlap(s1: i64, e1: i64, s2: i64, e2: i64) -> Option<(i64, i64)> {
    if s1 > e2 || s2 > e1 || s1 > e1 || s2 > e2 {
        None
    } else {
        let s = s1.max(s2);
        let e = e1.min(e2);
        if s > e { None } else { Some((s, e)) }
    }
}
fn get(x1: i64, p1: i64, s1: i64, e1: i64, x2: i64, p2: i64, s2: i64, e2: i64) -> i64 {
    if let Some((s, e)) = overlap(s1, e1, s2, e2) {
        let x1 = match p1 { -1 => x1 - s + s1, 0 => x1, _ => x1 + s - s1 };
        let x2 = match p2 { -1 => x2 - s + s2, 0 => x2, _ => x2 + s - s2 };
        case(x1, p1, x2, p2, e - s + 1)
    } else {
        0
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        // quadrant 1
        let (x1, y1, x2, y2) = (
            next::<i64>(&mut it), next::<i64>(&mut it),
            next::<i64>(&mut it), next::<i64>(&mut it));
        let (s1x1, s1y1, s1x2, s1y2) = (x1, y1, x2, y2);

        // quadrant 2
        let (x1, y1, x2, y2) = (
            next::<i64>(&mut it), next::<i64>(&mut it),
            next::<i64>(&mut it), next::<i64>(&mut it));
        let (s2x1, s2y1, s2x2, s2y2) = (x2, y1, x1, y2);

        // quadrant 3
        let (x1, y1, x2, y2) = (
            next::<i64>(&mut it), next::<i64>(&mut it),
            next::<i64>(&mut it), next::<i64>(&mut it));
        let (s3x1, s3y1, s3x2, s3y2) = (x2, y2, x1, y1);

        // quadrant 4
        let (x1, y1, x2, y2) = (
            next::<i64>(&mut it), next::<i64>(&mut it),
            next::<i64>(&mut it), next::<i64>(&mut it));
        let (s4x1, s4y1, s4x2, s4y2) = (x1, y2, x2, y1);

        // cut squares
        let (ax1, ay1, ax2, ay2) = (
            if s1x1 > s4x1 { s1x1 } else { s4x1 },
            if s1y1 > s2y1 { s1y1 } else { s2y1 },
            if s1x2 < s4x2 { s1x2 } else { s4x2 },
            if s1y2 < s2y2 { s1y2 } else { s2y2 });
        if ax1 > ax2 || ay1 > ay2 {
            writeln!(so, "0")?;
            continue;
        }
        let (bx1, by1, bx2, by2) = (
            if s3x1 > s2x1 { s3x1 } else { s2x1 },
            if s3y1 > s4y1 { s3y1 } else { s4y1 },
            if s3x2 < s2x2 { s3x2 } else { s2x2 },
            if s3y2 < s4y2 { s3y2 } else { s4y2 });
        if bx1 > bx2 || by1 > by2 {
            writeln!(so, "0")?;
            continue;
        }

        let mut k1 = vec![ax1 - ay1, ax1 - ay2, ax2 - ay1, ax2 - ay2];
        let mut k2 = vec![bx1 - by1, bx1 - by2, bx2 - by1, bx2 - by2];
        k1.sort(); k2.sort();

        let mut ans = 0;
        ans += get(1, 1, k1[0], k1[1]-1, 1, 1, k2[0], k2[1]-1);
        ans += get(1, 1, k1[0], k1[1]-1, k2[1]-k2[0]+1, 0, k2[1], k2[2]);
        ans += get(1, 1, k1[0], k1[1]-1, k2[1]-k2[0], -1, k2[2]+1, k2[3]);
        ans += get(k1[1]-k1[0]+1, 0, k1[1], k1[2], 1, 1, k2[0], k2[1]-1);
        ans += get(k1[1]-k1[0]+1, 0, k1[1], k1[2], k2[1]-k2[0]+1, 0, k2[1], k2[2]);
        ans += get(k1[1]-k1[0]+1, 0, k1[1], k1[2], k2[1]-k2[0], -1, k2[2]+1, k2[3]);
        ans += get(k1[1]-k1[0], -1, k1[2]+1, k1[3], 1, 1, k2[0], k2[1]-1);
        ans += get(k1[1]-k1[0], -1, k1[2]+1, k1[3], k2[1]-k2[0]+1, 0, k2[1], k2[2]);
        ans += get(k1[1]-k1[0], -1, k1[2]+1, k1[3], k2[1]-k2[0], -1, k2[2]+1, k2[3]);
        writeln!(so, "{}", ans)?;
    }

    Ok(())
}
