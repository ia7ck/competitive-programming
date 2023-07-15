use proconio::input;
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        boxes: [(u32, u32, u32); n],
    };

    let mut values = Vec::new();
    for &(h, w, d) in &boxes {
        values.push(h);
        values.push(w);
        values.push(d);
    }
    values.sort();
    values.dedup();
    let ord = |x: u32| -> usize {
        values.binary_search(&x).unwrap()
    };
    let m = values.len();
    let mut h_wd = vec![vec![]; m];
    for (h, w, d) in boxes {
        let mut v = [h, w, d];
        v.sort();
        let (h, w, d) = (ord(v[0]), ord(v[1]), ord(v[2]));
        h_wd[h].push((w, d));
    }
    for h in 0..m {
        h_wd[h].sort();
    }
    const INF: usize = std::usize::MAX / 2;
    let mut seg = SegmentTree::new(m, INF, |a, b| *a.min(b));
    for h in 0..m {
        for &(w, d) in &h_wd[h] {
            let min_d = seg.fold(0..w);
            if min_d < d {
                println!("Yes");
                return;
            }
        }
        for &(w, d) in &h_wd[h] {
            let &dd = seg.get(w);
            seg.update(w, d.min(dd));
        }
    }
    println!("No");
}
