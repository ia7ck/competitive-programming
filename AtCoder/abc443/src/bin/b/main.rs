use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
    };

    let mut total = 0;
    for i in 0.. {
        total += n + i;
        if total >= k {
            println!("{}", i);
            return;
        }
    }

    unreachable!();
}
