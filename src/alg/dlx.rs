// DLX - Knuth's Algorithm X with Dancing Links
#[derive(Default, Clone)] struct Node {
    id: usize,
    sz: usize,
    h: usize, u: usize, d: usize, l: usize, r: usize
}
#[inline] fn unlink_hor(dl: &mut Vec<Node>, id: usize) {
    let (l, r) = (dl[id].l, dl[id].r);
    dl[l].r = r; dl[r].l = l;
}
#[inline] fn unlink_ver(dl: &mut Vec<Node>, id: usize) {
    let (u, d, h) = (dl[id].u, dl[id].d, dl[id].h);
    dl[u].d = d; dl[d].u = u; dl[h].sz -= 1;
}
#[inline] fn relink_hor(dl: &mut Vec<Node>, id: usize) {
    let (l, r) = (dl[id].l, dl[id].r);
    dl[l].r = id; dl[r].l = id;
}
#[inline] fn relink_ver(dl: &mut Vec<Node>, id: usize) {
    let (u, d, h) = (dl[id].u, dl[id].d, dl[id].h);
    dl[u].d = id; dl[d].u = id; dl[h].sz += 1;
}
fn dlx_build(mat: Vec<Vec<bool>>, n: usize, m: usize) -> Vec<Node> {
    let mut dl = vec![Node::default(); n*m+m+1];

    // link header row
    dl[0].l = m; dl[0].r = 1;
    for j in 1..m {
        dl[j].l = j-1;
        dl[j].r = j+1;
    }
    dl[m].l = m-1; dl[m].r = 0;

    // link rows
    for i in 1..=n {
        dl[i*m+1].l = (i+1)*m; dl[i*m+1].r = i*m+2;
        for j in 2..m {
            dl[i*m+j].l = i*m+j-1;
            dl[i*m+j].r = i*m+j+1;
        }
        dl[(i+1)*m].l = (i+1)*m-1; dl[(i+1)*m].r = i*m+1;
    }

    // link columns
    for j in 1..=m {
        dl[j].u = j+n*m; dl[j].d = j+m; dl[j].sz = n;
        for i in 1..n {
            dl[i*m+j].h = j;
            dl[i*m+j].u = (i-1)*m+j;
            dl[i*m+j].d = (i+1)*m+j;
        }
        dl[n*m+j].u = (n-1)*m+j; dl[n*m+j].d = j; dl[n*m+j].h = j;
    }

    // unlink 0s
    for i in 1..=n {
        for j in 1..=m {
            let id = i*m+j;
            if !mat[i-1][j-1] {
                unlink_hor(&mut dl, id);
                unlink_ver(&mut dl, id);
            }
        }
    }

    dl
}
fn dlx_cover(dl: &mut Vec<Node>, h: usize) {
    unlink_hor(dl, h);
    let mut i = dl[h].d;
    while i != h {
        let mut j = dl[i].r;
        while j != i {
            unlink_ver(dl, j);
            j = dl[j].r;
        }
        i = dl[i].d;
    }
}
fn dlx_uncover(dl: &mut Vec<Node>, h: usize) {
    let mut i = dl[h].u;
    while i != h {
        let mut j = dl[i].l;
        while j != i {
            relink_ver(dl, j);
            j = dl[j].l;
        }
        i = dl[i].u;
    }
    relink_hor(dl, h);
}
fn dlx(dl: &mut Vec<Node>, sol: &mut Vec<usize>) -> bool {
    if dl[0].r == 0 { return true; }

    // find column with minimum size
    let mut h = dl[0].r;
    let (mut mh, mut ms) = (h, dl[h].sz);
    while h != 0 {
        if dl[h].sz == 0 { return false; }
        if dl[h].sz < ms {
            mh = h; ms = dl[h].sz;
        }
        h = dl[h].r;
    }

    // cover
    dlx_cover(dl, mh);
    let mut i = dl[mh].d;
    while i != mh {
        // push solution
        sol.push(i);
        // cover columns
        let mut j = dl[i].r;
        while j != i {
            dlx_cover(dl, dl[j].h);
            j = dl[j].r;
        }
        // recursion
        if dlx(dl, sol) { return true; }
        else { // backtrack
            sol.pop();
            let mut j = dl[i].l;
            while j != i {
                dlx_uncover(dl, dl[j].h);
                j = dl[j].l;
            }
        }
        i = dl[i].d;
    }

    dlx_uncover(dl, mh);
    false
}