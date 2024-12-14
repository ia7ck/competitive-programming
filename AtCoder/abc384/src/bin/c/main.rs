use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        score: [u32; 5],
    };

    let mut ans = Vec::new();
    for bits in 1..(1 << 5) {
        let mut name = String::new();
        let mut total = 0;
        for i in 0..5 {
            if bits >> i & 1 == 1 {
                name.push(char::from(b'A' + i as u8));
                total += score[i];
            }
        }
        ans.push((name, total));
    }

    ans.sort_by_key(|(name, total)| (Reverse(*total), name.clone()));
    for (ans, _) in ans {
        println!("{}", ans);
    }
}
