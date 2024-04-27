use proconio::input;

fn main() {
    input! {
        a: [u32; 9],
        b: [u32; 8],
    };

    let a_sum = a.iter().sum::<u32>();
    let b_sum = b.iter().sum::<u32>();

    let ans = a_sum - b_sum + 1;
    println!("{}", ans);
}
