use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(u32, u8); n],
    };   

    let mut ans = 0;
    for start in 0..24 {
        let mut sum = 0;
        for &(w, x) in &wx {
            let local = (start + x) % 24;
            if local >= 9 && local <= 17 {
                sum += w;
            }
        }
        ans = ans.max(sum);
    }
    println!("{}", ans);
}
