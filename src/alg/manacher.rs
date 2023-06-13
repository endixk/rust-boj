fn manacher(s: &[u8], n: usize) -> Vec<usize> {
    let mut a = vec![0; n];
    let (mut r, mut p) = (0, 0);
    for i in 0..n {
        if i <= r {
            a[i] = a[2 * p - i].min(r - i);
        }
        while i > a[i] && i + a[i] + 1 < n && s[i - a[i] - 1] == s[i + a[i] + 1] {
            a[i] += 1;
        }
        if r < i + a[i] {
            r = i + a[i];
            p = i;
        }
    }
    a
}