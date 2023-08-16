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

use std::collections::HashMap;

// first digit of x
#[inline]
fn first_digit(x: usize) -> usize {
    x / 10usize.pow((x as f64).log10() as u32)
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (g, r, b) = (
        next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it)
    );
    let (mut gmap, mut rmap, mut bmap) = (
        HashMap::new(), HashMap::new(), HashMap::new()
    );
    for _ in 0..g { let x = next::<usize>(&mut it); *gmap.entry(x).or_insert(0i64) += 1; }
    for _ in 0..r { let x = next::<usize>(&mut it); *rmap.entry(x).or_insert(0i64) += 1; }
    for _ in 0..b { let x = next::<usize>(&mut it); *bmap.entry(x).or_insert(0i64) += 1; }

    let mut gsuff = [0; 10]; // G classified by last digit
    for (k, v) in gmap.iter() { gsuff[k % 10] += v; }
    let mut bpref = [0; 10]; // B classified by first digit
    let mut gbid = [0; 10]; // Identical pairs of G and B
    let mut gbdf = [[0; 10]; 10]; // Different pairs of G and B
    for (k, v) in bmap.iter() {
        let (pref, suff) = (first_digit(*k), k % 10);
        bpref[first_digit(*k)] += v;
        let &x = gmap.get(k).unwrap_or(&0);
        if x > 0 {
            if pref == suff {
                gbid[pref] += v * x;
            } else {
                gbdf[suff][pref] += v * x;
            }
        }
    }

    let mut ans = 0;
    for (k, v) in rmap.iter() {
        let (pref, suff) = (first_digit(*k), k % 10);
        ans += v * gsuff[pref] * bpref[suff];
        if pref == suff {
            let (&x, &y) = (gmap.get(k).unwrap_or(&0), bmap.get(k).unwrap_or(&0));
            ans -= v * (x * bpref[suff] + y * gsuff[pref] + gbid[pref] - 2 * x * y);
        } else {
            ans -= v * gbdf[pref][suff];
        }
    }
    writeln!(so, "{}", ans).ok();
}
