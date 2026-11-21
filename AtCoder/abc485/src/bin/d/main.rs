use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for &x in &s {
        match x {
            'A' => a += 1,
            'B' => b += 1,
            'C' => c += 1,
            _ => unreachable!(),
        }
    }

    if a == 0 || b == 0 || c == 0 {
        println!("0");
        return;
    }

    let first = {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        (0..n)
            .find(|&i| {
                match s[i] {
                    'A' => a += 1,
                    'B' => b += 1,
                    'C' => c += 1,
                    _ => unreachable!(),
                }

                a >= 1 && b >= 1 && c >= 1
            })
            .unwrap()
    };

    let rev_first = {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        (0..n)
            .rfind(|&i| {
                match s[i] {
                    'A' => a += 1,
                    'B' => b += 1,
                    'C' => c += 1,
                    _ => unreachable!(),
                }

                a >= 1 && b >= 1 && c >= 1
            })
            .unwrap()
    };

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut i = 0;
    let mut ans = 0;
    for j in 0..n {
        match s[j] {
            'A' => a += 1,
            'B' => b += 1,
            'C' => c += 1,
            _ => unreachable!(),
        }

        while i < j {
            let mut new_a = a;
            let mut new_b = b;
            let mut new_c = c;
            match s[i] {
                'A' => new_a -= 1,
                'B' => new_b -= 1,
                'C' => new_c -= 1,
                _ => unreachable!(),
            }
            if new_a >= 1 && new_b >= 1 && new_c >= 1 {
                (a, b, c) = (new_a, new_b, new_c);
                i += 1;
            } else {
                break;
            }
        }

        if a >= 1 && b >= 1 && c >= 1 {
            if first < i && j < rev_first {
                ans += i - first;
            }
        }
    }

    println!("{}", ans);
}
