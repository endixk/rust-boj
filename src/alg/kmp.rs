fn pi(s: &String) -> Vec<usize> {
    let (s, l) = (s.as_bytes(), s.len());
    let mut p = vec![0; l];

    let mut j = 0;
    for i in 1..l {
        while j > 0 && s[i] != s[j] { j = p[j-1]; }
        if s[i] == s[j] { j += 1; p[i] = j; }
    }

    p
}

fn kmp(s: &String, t: &String) -> u32 {
    let p = pi(t);
    let (s, t) = (s.as_bytes(), t.as_bytes());
    let (n, m) = (s.len(), t.len());

    let (mut occ, mut j) = (0, 0);
    for i in 0..n {
        while j > 0 && s[i] != t[j] { j = p[j-1]; }
        if s[i] == t[j] {
            if j == m-1 { occ += 1; j = p[j]; }
            else { j += 1; }
        }
    }
    occ
}

fn z(s: &String) -> Vec<usize> {
    let (s, n) = (s.as_bytes(), s.len());
    let mut z = vec![0; n];
    z[0] = n;

    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i > r {
            l = i; r = i;
            while r < n && s[r-l] == s[r] { r += 1; }
            z[i] = r-l; r -= 1;
        } else {
            let k = i-l;
            if z[k] < r-i+1 { z[i] = z[k]; }
            else {
                l = i;
                while r < n && s[r-l] == s[r] { r += 1; }
                z[i] = r-l; r -= 1;
            }
        }
    }

    z
}