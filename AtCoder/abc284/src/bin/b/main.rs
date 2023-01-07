use proconio::input;

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            a: [u32; n],
        };

        let ans = a.into_iter().filter(|x| x % 2 == 1).count();
        println!("{}", ans);
    }
}
