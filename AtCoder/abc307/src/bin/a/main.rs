use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n * 7],
    };

    for i in 0..n {
        let ans = a[i * 7..(i + 1) * 7].iter().sum::<u32>();
        print!("{}", ans);
        if i + 1 < n {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
