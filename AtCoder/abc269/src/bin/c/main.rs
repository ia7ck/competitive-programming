use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    let mut positions = Vec::new();
    for i in 0..60 {
        if n >> i & 1 == 1 {
            positions.push(i);
        }
    }

    let mut ans = Vec::new();
    for bits in 0..(1 << positions.len()) {
        let mut x = 0;
        for i in 0..positions.len() {
            if bits >> i & 1 == 1 {
                x ^= 1_u64 << positions[i];
            }
        }
        ans.push(x);
    }

    ans.sort();
    for ans in ans {
        println!("{}", ans);
    }
}
