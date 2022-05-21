use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        slots: [Chars; n],
    };

    let mut ans = std::usize::MAX;
    for d in 0..=9 {
        let c = ('0' as u8 + d) as char;
        let mut t = vec![0; 10];
        for s in &slots {
            let i = s.iter().position(|&ch| ch == c).unwrap();
            t[i] += 1;
        }
        let max = t
            .iter()
            .enumerate()
            .map(|(k, &t)| if t == 0 { 0 } else { k + (t - 1) * 10 })
            .max()
            .unwrap();
        ans = ans.min(max);
    }

    println!("{}", ans);
}
