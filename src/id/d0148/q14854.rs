// BOJ 14854 [Binomial Coefficient 6]
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

fn fr(m: i64) -> (Vec<i64>, Vec<i64>) {
    let mut dp = vec![m; m as usize + 1];
    dp[1] = 1;
    (2..=m).for_each(|i| { dp[i as usize] = -(m / i) * dp[(m % i) as usize] % m; });

    let mut f = vec![1; m as usize + 1];
    let mut r = vec![1; m as usize + 1];
    (1..=m).for_each(|i| {
        f[i as usize] = f[i as usize - 1] * i % m;
        r[i as usize] = r[i as usize - 1] * dp[i as usize] % m;
    });
    (f, r)
}
fn nck(n: i64, k: i64, m: i64, f: &[i64], r: &[i64]) -> i64 {
    let mut ans = 1;
    let (mut n, mut k) = (n, k);
    while n > 0 || k > 0 {
        let (a, b) = ((n % m) as usize, (k % m) as usize);
        if a > b {
            ans = ans * f[a] * r[b] * r[a-b] % m;
        } else { return 0; }
        n /= m;
        k /= m;
    }
    (ans + m) % m
}

fn pmod(a: i64, x: i64, m: i64) -> i64 {
    let mut ret = 1;
    let mut a = a % m;
    let mut x = x;
    while x > 0 {
        if x & 1 == 1 { ret = ret * a % m; }
        a = a * a % m;
        x >>= 1;
    }
    ret
}
fn mmi(a: i64, m: i64) -> i64 {
    pmod(a % m, m - 2, m)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (f11, r11) = fr(11);
    let (f13, r13) = fr(13);
    let (f27, r27) = fr(27);
    let (f37, r37) = fr(37);

    for _ in 0..next(&mut it) {
        let (n, k) = (next(&mut it), next(&mut it));
        let (a, b, c, d) = (
            nck(n, k, 11, &f11, &r11),
            nck(n, k, 13, &f13, &r13),
            nck(n, k, 27, &f27, &r27),
            nck(n, k, 37, &f37, &r37),
        );

        let x = a * 13*27*37 * mmi(13*27*37, 11) +
            b * 11*27*37 * mmi(11*27*37, 13) +
            c * 11*13*37 * mmi(11*13*37, 27) +
            d * 11*13*27 * mmi(11*13*27, 37);
        writeln!(so, "{}", x % 142857).unwrap();
    }
}
