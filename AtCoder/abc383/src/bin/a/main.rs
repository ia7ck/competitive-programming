use proconio::input;

fn main() {
    input! {
        n: usize,
        tv: [(u32, u32); n],
    };

    let (_, mut ans) = tv[0];
    for i in 1..n {
        let (t1, _) = tv[i - 1];
        let (t2, v) = tv[i];
        ans = ans.saturating_sub(t2 - t1) + v;
    }

    println!("{}", ans);
}
