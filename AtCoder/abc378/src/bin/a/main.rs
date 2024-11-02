use proconio::input;

fn main() {
    input! {
        a: [usize; 4],
    };

    let mut count = [0; 5];
    for x in a {
        count[x] += 1;
    }
    let mut ans = 0;
    for i in 1..=4 {
        ans += count[i] / 2;
    }

    println!("{}", ans);
}
