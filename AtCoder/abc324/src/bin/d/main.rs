use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        s: Bytes,
    };

    let mut f = [0; 10];
    for d in s {
        f[(d - b'0') as usize] += 1;
    }

    let mut ans = 0;
    for x in 0..10_000_000_u64 {
        let x = x * x;
        if format!("{}", x).len() > n {
            break;
        }
        let mut g = [0; 10];
        for d in format!("{:0n$}", x).bytes() {
            g[(d - b'0') as usize] += 1;
        }
        if f == g {
            ans += 1;
        }
    }
    println!("{}", ans);
}
