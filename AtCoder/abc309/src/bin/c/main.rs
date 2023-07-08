use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        mut ab: [(u64, u64); n],
    };

    let mut sum = 0;
    for &(_, b) in &ab {
        sum += b;
    }
    if sum <= k {
        println!("1");
        return;
    }
    ab.sort();
    for (a, b) in ab {
        sum -= b;
        if sum <= k {
            println!("{}", a + 1);
            return;
        }
    }
    unreachable!();
}
