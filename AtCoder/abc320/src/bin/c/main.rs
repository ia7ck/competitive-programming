use proconio::{input, marker::Bytes};

fn main() {
    input! {
        m: usize,
        s: [Bytes; 3],
    };

    let mut ans = usize::MAX;
    for d in b'0'..=b'9' {
        for (o1, o2, o3) in [(0, 1, 2), (0, 2, 1), (1, 0, 2), (1, 2, 0), (2, 0, 1), (2, 1, 0)] {
            let mut t = 0;
            if s[o1].contains(&d) == false {
                continue;
            }
            while s[o1][t % m] != d {
                t += 1;
            }
            t += 1;
            if s[o2].contains(&d) == false {
                continue;
            }
            while s[o2][t % m] != d {
                t += 1;
            }
            t += 1;
            if s[o3].contains(&d) == false {
                continue;
            }
            while s[o3][t % m] != d {
                t += 1;
            }
            ans = ans.min(t);
        }
    }
    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
