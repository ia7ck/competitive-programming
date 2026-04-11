use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i64; n],
    };

    let mut ans = 0;
    for bits in 0..(1 << n) {
        let mut p = 1;
        let mut z = 0;
        for (i, &l) in l.iter().enumerate() {
            let l = l * 2;
            let new_p = if bits >> i & 1 == 1 { p + l } else { p - l };
            if (p < 0 && new_p > 0) || (p > 0 && new_p < 0) {
                z += 1;
            }
            p = new_p;
        }
        ans = ans.max(z);
    }

    println!("{}", ans);
}
