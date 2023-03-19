use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let ans = a.into_iter().filter(|&x| x % 2 == 0).collect::<Vec<_>>();
    for i in 0..ans.len() {
        print!("{}", ans[i]);
        if i + 1 < ans.len() {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
