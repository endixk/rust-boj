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

fn union_sz(root: &mut Vec<usize>,
              rank: &mut Vec<usize>,
              size: &mut Vec<usize>, x: usize, y: usize) {
    let x = find(root, x);
    let y = find(root, y);
    if x != y {
        if rank[x] < rank[y] {
            root[x] = y;
            size[y] += size[x];
        } else {
            root[y] = x;
            size[x] += size[y];
            if rank[x] == rank[y] {
                rank[x] += 1;
            }
        }
    }
}

fn union_size(root: &mut Vec<usize>,
              rank: &mut Vec<usize>,
              size: &mut Vec<usize>, x: usize, y: usize) -> usize {
    let x = find(root, x);
    let y = find(root, y);
    if x != y {
        if rank[x] < rank[y] {
            root[x] = y;
            size[y] += size[x];
            size[y]
        } else {
            root[y] = x;
            size[x] += size[y];
            if rank[x] == rank[y] {
                rank[x] += 1;
            }
            size[x]
        }
    } else {
        size[x]
    }
}