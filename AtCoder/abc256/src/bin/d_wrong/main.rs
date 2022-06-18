use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(u32, u32); n],
    };

    lr.sort();
    lr.dedup();

    let mut ans = Vec::new();
    let (mut ll, mut rr) = lr[0];
    for &(l, r) in &lr[1..] {
        if rr < l {
            ans.push((ll, rr));
            ll = l;
            rr = r;
        } else {
            rr = r;
        }
    }
    ans.push((ll, rr));

    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}
