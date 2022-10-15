use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut b = a.clone();
    b.sort();
    b.dedup();
    let mut ans = vec![0; n];
    for x in a {
        let i = b.binary_search(&x).unwrap();
        ans[b.len() - i - 1] += 1;
    }
    for ans in ans {
        println!("{}", ans);
    }
}
