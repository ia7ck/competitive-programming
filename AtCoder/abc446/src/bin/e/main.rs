use proconio::input;

fn main() {
    input! {
        m: usize,
        a: usize,
        b: usize,
    };

    let mut ans = 0;
    let mut div_m = vec![vec![Option::<bool>::None; m]; m];
    for x in 0..m {
        for y in 0..m {
            if x == 0 || y == 0 {
                div_m[x][y] = Some(true);
                continue;
            }
            let mut s = vec![x, y];
            let mut hit = false;
            let mut div = false;
            for i in 2..=(m * 2) {
                let t = (a * s[i - 1] + b * s[i - 2]) % m;
                s.push(t);
                if let Some(d) = div_m[s[i - 1]][s[i]] {
                    hit = true;
                    if !d {
                        ans += 1;
                    }
                    break;
                }
                if t == 0 {
                    div = true;
                    break;
                }
            }
            if !hit {
                for w in s.windows(2) {
                    div_m[w[0]][w[1]] = Some(div);
                }
                if !div {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
