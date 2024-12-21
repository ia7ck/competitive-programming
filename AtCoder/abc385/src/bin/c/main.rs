use proconio::input;
use run_length::RunLength;

fn main() {
    input! {
        n: usize,
        h: [u32; n],
    };

    let mut ans = 1;
    for i in 0..n {
        for w in 1..n {
            let a = h.iter().skip(i).step_by(w).copied().collect::<Vec<_>>();
            for (_, len) in RunLength::new(&a) {
                ans = ans.max(len);
            }
        }
    }

    println!("{}", ans);
}
