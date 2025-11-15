use proconio::input;

fn main() {
    input! {
        mut v: [u32; 3],
    };

    v.sort_unstable();
    v.reverse();

    let mut ans = 0;
    for v in v {
        ans = ans * 10 + v;
    }
    println!("{}", ans);
}
