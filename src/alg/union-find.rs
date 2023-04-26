fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x {
        x
    } else {
        root[x] = find(root, root[x]);
        root[x]
    }
}

fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let x = find(root, x);
    let y = find(root, y);
    if x == y {
        return;
    }
    if rank[x] < rank[y] {
        root[x] = y;
    } else {
        root[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}

fn union_cnt(root: &mut Vec<usize>,
             rank: &mut Vec<usize>,
             cnt: &mut Vec<usize>, x: usize, y: usize) -> usize {
    let x = find(root, x);
    let y = find(root, y);
    if x != y {
        if rank[x] < rank[y] {
            root[x] = y;
            cnt[y] += cnt[x];
            cnt[x] = 1;
            cnt[y]
        } else {
            root[y] = x;
            cnt[x] += cnt[y];
            cnt[y] = 1;
            if rank[x] == rank[y] {
                rank[x] += 1;
            }
            cnt[x]
        }
    } else {
        cnt[x]
    }
}