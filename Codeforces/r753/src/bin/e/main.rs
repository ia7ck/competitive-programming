use input_i_scanner::InputIScanner;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    macro_rules! scan {
        (($($t: ty),+)) => {
            ($(scan!($t)),+)
        };
        ($t: ty) => {
            _i_i.scan::<$t>() as $t
        };
        (($($t: ty),+); $n: expr) => {
            std::iter::repeat_with(|| scan!(($($t),+))).take($n).collect::<Vec<_>>()
        };
        ($t: ty; $n: expr) => {
            std::iter::repeat_with(|| scan!($t)).take($n).collect::<Vec<_>>()
        };
    }

    let t = scan!(usize);
    for _ in 0..t {
        let (h, w) = scan!((usize, usize));
        let s = scan!(String);
        solve(h, w, s);
    }
}

fn solve(h: usize, w: usize, s: String) {
    let s: Vec<char> = s.chars().collect();
    let row_ok = |n: usize| -> Option<usize> {
        let mut top = 1;
        let mut bottom = h;
        for &ch in s.iter().take(n).rev() {
            if ch == 'D' {
                if bottom == 1 {
                    return None;
                }
                top = 1.max(top - 1);
                bottom -= 1;
            }  else if ch == 'U' {
                if top == h {
                    return None;
                }
                top += 1;
                bottom = h.min(bottom + 1);
            }
        }
        assert!(top <= bottom);
        Some(top)
    };
    let col_ok = |n: usize| -> Option<usize> {
        let mut left = 1;
        let mut right = w;
        for &ch in s.iter().take(n).rev() {
            if ch == 'R' {
                if right == 1 {
                    return None;
                }
                left = 1.max(left - 1);
                right -= 1;
            }  else if ch == 'L' {
                if left == w {
                    return None;
                }
                left += 1;
                right = w.min(right + 1);
            }
        }
        assert!(left <= right);
        Some(left)
    };

    let mut ok = 0;
    let mut ng = s.len() + 1;
    let mut ans = (1, 1);
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let r = row_ok(mid);
        let c = col_ok(mid);
        match (r, c) {
            (Some(r), Some(c)) => {
                ok = mid;
                ans = (r, c);
            }
            _ => {
                ng = mid;
            }
        }
    }
    dbg!(ok);
    println!("{} {}", ans.0, ans.1);
}
