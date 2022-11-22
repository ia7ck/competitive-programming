use proconio::input;

fn pow(a: u64, x: u64, m: u64) -> u64 {
    if x == 0 {
        1
    } else if x % 2 == 0 {
        pow(a * a % m, x / 2, m)
    } else {
        a * pow(a, x - 1, m) % m
    }
}

// fn solve(c: u64, d: u64, m: u64) -> u64 {
//     if d == 1 {
//         return c % m;
//     }

//     // c = 7, d = 10
//     // 7777777777
//     //      ^^^^^

//     if d % 2 == 0 {
//         let b = solve(c, d / 2, m);
//         (b * pow(10, d / 2, m) + b) % m
//     } else {
//         let front = (d - 1) / 2;
//         let back = (d + 1) / 2;
//         (solve(c, front, m) * pow(10, back, m) + solve(c, back, m)) % m
//     }
// }

fn main() {
    input! {
        k: usize,
        m: u64,
        cd: [(u64, u64); k],
    };

    const D: usize = 40;
    let mut b = vec![vec![0; D]; 10]; // b[c][i] := c を 2^i 個連結してできる整数 mod m
    for c in 0..10 {
        b[c][0] = c as u64 % m;
        for i in 1..D {
            b[c][i] = (b[c][i - 1] * pow(10, 1 << (i - 1), m) + b[c][i - 1]) % m;
        }
    }

    // O(K * log(max(d))^2)
    let mut ans = 0;
    for (c, d) in cd {
        let mut sub = 0;
        for j in 0..D {
            if d >> j & 1 == 1 {
                sub = (sub * pow(10, 1 << j, m) + b[c as usize][j]) % m;
            }
        }
        ans = (ans * pow(10, d, m) + sub) % m;
        // ans = (ans * pow(10, d, m) + solve(c, d, m)) % m;
    }
    println!("{}", ans);
}
