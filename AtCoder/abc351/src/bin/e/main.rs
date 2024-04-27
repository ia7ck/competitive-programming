use proconio::input;
use segment_tree::SegmentTree;
use zarts::SortedSeq;

fn main() {
    input! {
        n: usize,
        points: [(i64, i64); n],
    };

    const D: i64 = 100_000_000;

    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for &(x, y) in &points {
        xs.push(x + y + D);
        ys.push(x - y + D);
    }
    let xs = SortedSeq::from_iter(xs.into_iter());
    let ys = SortedSeq::from_iter(ys.into_iter());

    let mut cnt_x_00 = SegmentTree::new(xs.size(), 0, |a, b| a + b);
    let mut cnt_x_01 = SegmentTree::new(xs.size(), 0, |a, b| a + b);
    let mut cnt_x_10 = SegmentTree::new(xs.size(), 0, |a, b| a + b);
    let mut cnt_x_11 = SegmentTree::new(xs.size(), 0, |a, b| a + b);
    let mut cnt_y_00 = SegmentTree::new(ys.size(), 0, |a, b| a + b);
    let mut cnt_y_01 = SegmentTree::new(ys.size(), 0, |a, b| a + b);
    let mut cnt_y_10 = SegmentTree::new(ys.size(), 0, |a, b| a + b);
    let mut cnt_y_11 = SegmentTree::new(ys.size(), 0, |a, b| a + b);
    let mut val_x_00 = SegmentTree::new(xs.size(), 0, |a, b| a + b);
    let mut val_x_01 = SegmentTree::new(xs.size(), 0, |a, b| a + b);
    let mut val_x_10 = SegmentTree::new(xs.size(), 0, |a, b| a + b);
    let mut val_x_11 = SegmentTree::new(xs.size(), 0, |a, b| a + b);
    let mut val_y_00 = SegmentTree::new(ys.size(), 0, |a, b| a + b);
    let mut val_y_01 = SegmentTree::new(ys.size(), 0, |a, b| a + b);
    let mut val_y_10 = SegmentTree::new(ys.size(), 0, |a, b| a + b);
    let mut val_y_11 = SegmentTree::new(ys.size(), 0, |a, b| a + b);
    for &(x, y) in &points {
        let (x, y) = (x + y + D, x - y + D);
        let i = xs.ord(&x);
        let j = ys.ord(&y);
        match (x % 2, y % 2) {
            (0, 0) => {
                cnt_x_00.update(i, cnt_x_00.get(i) + 1);
                cnt_y_00.update(j, cnt_y_00.get(j) + 1);
                val_x_00.update(i, val_x_00.get(i) + x);
                val_y_00.update(j, val_y_00.get(j) + y);
            }
            (0, 1) => {
                cnt_x_01.update(i, cnt_x_01.get(i) + 1);
                cnt_y_01.update(j, cnt_y_01.get(j) + 1);
                val_x_01.update(i, val_x_01.get(i) + x);
                val_y_01.update(j, val_y_01.get(j) + y);
            }
            (1, 0) => {
                cnt_x_10.update(i, cnt_x_10.get(i) + 1);
                cnt_y_10.update(j, cnt_y_10.get(j) + 1);
                val_x_10.update(i, val_x_10.get(i) + x);
                val_y_10.update(j, val_y_10.get(j) + y);
            }
            (1, 1) => {
                cnt_x_11.update(i, cnt_x_11.get(i) + 1);
                cnt_y_11.update(j, cnt_y_11.get(j) + 1);
                val_x_11.update(i, val_x_11.get(i) + x);
                val_y_11.update(j, val_y_11.get(j) + y);
            }
            _ => unreachable!(),
        }
    }

    let mut ans = 0;
    for &(x, y) in &points {
        let (x, y) = (x + y + D, x - y + D);
        let i = xs.ord(&x);
        let j = ys.ord(&y);
        match (x % 2, y % 2) {
            (0, 0) => {
                ans += ((cnt_x_00.fold(0..i) * x - val_x_00.fold(0..i))
                    + (val_x_00.fold((i + 1)..xs.size()) - cnt_x_00.fold((i + 1)..xs.size()) * x))
                    / 2;
                ans += ((cnt_y_00.fold(0..j) * y - val_y_00.fold(0..j))
                    + (val_y_00.fold((j + 1)..ys.size()) - cnt_y_00.fold((j + 1)..ys.size()) * y))
                    / 2;
                cnt_x_00.update(i, cnt_x_00.get(i) - 1);
                cnt_y_00.update(j, cnt_y_00.get(j) - 1);
                val_x_00.update(i, val_x_00.get(i) - x);
                val_y_00.update(j, val_y_00.get(j) - y);
            }
            (0, 1) => {
                ans += ((cnt_x_01.fold(0..i) * x - val_x_01.fold(0..i))
                    + (val_x_01.fold((i + 1)..xs.size()) - cnt_x_01.fold((i + 1)..xs.size()) * x))
                    / 2;
                ans += ((cnt_y_01.fold(0..j) * y - val_y_01.fold(0..j))
                    + (val_y_01.fold((j + 1)..ys.size()) - cnt_y_01.fold((j + 1)..ys.size()) * y))
                    / 2;
                cnt_x_01.update(i, cnt_x_01.get(i) - 1);
                cnt_y_01.update(j, cnt_y_01.get(j) - 1);
                val_x_01.update(i, val_x_01.get(i) - x);
                val_y_01.update(j, val_y_01.get(j) - y);
            }
            (1, 0) => {
                ans += ((cnt_x_10.fold(0..i) * x - val_x_10.fold(0..i))
                    + (val_x_10.fold((i + 1)..xs.size()) - cnt_x_10.fold((i + 1)..xs.size()) * x))
                    / 2;
                ans += ((cnt_y_10.fold(0..j) * y - val_y_10.fold(0..j))
                    + (val_y_10.fold((j + 1)..ys.size()) - cnt_y_10.fold((j + 1)..ys.size()) * y))
                    / 2;
                cnt_x_10.update(i, cnt_x_10.get(i) - 1);
                cnt_y_10.update(j, cnt_y_10.get(j) - 1);
                val_x_10.update(i, val_x_10.get(i) - x);
                val_y_10.update(j, val_y_10.get(j) - y);
            }
            (1, 1) => {
                ans += ((cnt_x_11.fold(0..i) * x - val_x_11.fold(0..i))
                    + (val_x_11.fold((i + 1)..xs.size()) - cnt_x_11.fold((i + 1)..xs.size()) * x))
                    / 2;
                ans += ((cnt_y_11.fold(0..j) * y - val_y_11.fold(0..j))
                    + (val_y_11.fold((j + 1)..ys.size()) - cnt_y_11.fold((j + 1)..ys.size()) * y))
                    / 2;
                cnt_x_11.update(i, cnt_x_11.get(i) - 1);
                cnt_y_11.update(j, cnt_y_11.get(j) - 1);
                val_x_11.update(i, val_x_11.get(i) - x);
                val_y_11.update(j, val_y_11.get(j) - y);
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
