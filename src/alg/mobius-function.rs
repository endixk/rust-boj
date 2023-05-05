// mobius O(n log n)
// mo = vec![0i8; sz];
fn mobius(mo: &mut Vec<i8>, sz: usize) {
    mo[1] = 1;
    for i in 1..sz {
        for j in (i<<1..sz).step_by(i) {
            mo[j] -= mo[i];
        }
    }
}

// mobius O(n log log n)
// mo = vec![1i8; sz];
fn mobius2(mo: &mut Vec<i8>, sz: usize) {
    let mut isp = vec![true; sz];
    for i in 2..sz {
        if isp[i] {
            for j in (i..sz).step_by(i) {
                isp[j] = false;
                mo[j] = -mo[j];
                if j * i < sz {
                    mo[j * i] = 0;
                }
            }
        }
    }
}