use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n == 1 {
        println!("0");
        return;
    }

    let even = [0, 2, 4, 6, 8];
    let mut ans = Vec::new();
    let mut n = n - 1;
    while n > 0 {
        ans.push(even[n % 5]);
        n /= 5;
    }
    ans.reverse();
    assert_ne!(ans[0], 0);
    for d in ans {
        print!("{}", d);
    }
    println!();
}

// 132 = 125 * 1 + 25 * 0 + 5 * 1 + 1 * 2

// 0, 2, 4, 6, 8
// 20, 22, 24, 26, 28
// 40, 42, 44, 46, 48
// 60, 62, 64, 66, 68
// 80, 82, 84, 86, 88
// 200, 202, 204, 206, 208
