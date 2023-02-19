use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    };

    a.sort();
    let mut ans = 0;
    for x in a {
        if ans == x {
            ans += 1;
        }
    }

    println!("{}", ans.min(k));
}
