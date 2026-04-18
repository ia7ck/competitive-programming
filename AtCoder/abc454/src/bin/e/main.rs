use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            a: Usize1,
            b: Usize1,
        };

        solve(n, a, b);
    }
}

fn solve(n: usize, a: usize, b: usize) {
    if n % 2 == 1 {
        println!("No");
        return;
    }

    if (a + b) % 2 == 0 {
        println!("No");
        return;
    }

    let mut trans = false;
    let (a, b) = if a < b {
        (a, b)
    } else {
        trans = true;
        (b, a)
    };

    let mut ans = Vec::new();
    if a % 2 == 0 {
        for i in 0..a {
            let dir = if i % 2 == 0 { 'R' } else { 'L' };
            for _ in 0..(n - 1) {
                ans.push(dir);
            }
            ans.push('D');
        }

        for j in 0..n {
            if j < b {
                let dir = if j % 2 == 0 { 'D' } else { 'U' };
                ans.push(dir);
                ans.push('R');
            } else if j == b {
                ans.push('R');
            } else {
                let dir = if j % 2 == 0 { 'U' } else { 'D' };
                ans.push(dir);
                ans.push('R');
            }
        }
        let last = ans.pop();
        assert_eq!(last, Some('R'));

        for i in (a + 2)..n {
            ans.push('D');
            let dir = if (i - (a + 2)) % 2 == 0 { 'L' } else { 'R' };
            for _ in 0..(n - 1) {
                ans.push(dir);
            }
        }
    } else {
        assert!(a >= 1);
        for i in 0..(a - 1) {
            let dir = if i % 2 == 0 { 'R' } else { 'L' };
            for _ in 0..(n - 1) {
                ans.push(dir);
            }
            ans.push('D');
        }

        for j in 0..n {
            if j < b {
                let dir = if j % 2 == 0 { 'D' } else { 'U' };
                ans.push(dir);
                ans.push('R');
            } else if j == b {
                ans.push('R');
            } else {
                let dir = if j % 2 == 0 { 'U' } else { 'D' };
                ans.push(dir);
                ans.push('R');
            }
        }
        let last = ans.pop();
        assert_eq!(last, Some('R'));

        for i in (a + 1)..n {
            ans.push('D');
            let dir = if (i - (a + 1)) % 2 == 0 { 'L' } else { 'R' };
            for _ in 0..(n - 1) {
                ans.push(dir);
            }
        }
    }

    if trans {
        for i in 0..ans.len() {
            ans[i] = match ans[i] {
                'D' => 'R',
                'R' => 'D',
                'U' => 'L',
                'L' => 'U',
                _ => unreachable!(),
            };
        }
    }

    println!("Yes");
    println!("{}", ans.iter().collect::<String>());
}
