use proconio::input;

const N: usize = 3;
fn dfs(a: &mut [[u8; N]; N], k: usize, h: &[u8; N], w: &[u8; N], ans: &mut usize) {
    if k == N * N {
        let mut ok = true;
        for i in 0..N {
            let s = a[i].iter().sum::<u8>();
            if s != h[i] {
                ok = false;
            }
        }
        for j in 0..N {
            let s = (0..N).map(|i| a[i][j]).sum::<u8>();
            if s != w[j] {
                ok = false;
            }
        }
        if ok {
            *ans += 1;
        }
        return;
    }
    let r = k / 3;
    let c = k % 3;
    let x = a[r][c];
    let mut left_sum = 0;
    for j in 0..c {
        left_sum += a[r][j];
    }
    let mut up_sum = 0;
    for i in 0..r {
        up_sum += a[i][c];
    }
    for y in 1..=30 {
        if left_sum + y <= h[r] && up_sum + y <= w[c] {
            let mut ok = true;
            if c + 1 == N {
                ok &= left_sum + y == h[r];
            }
            if r + 1 == N {
                ok &= up_sum + y == w[c];
            }
            if ok {
                a[r][c] = y;
                dfs(a, k + 1, h, w, ans);
                a[r][c] = x;
            }
        }
    }
}

fn main() {
    input! {
        h1: u8,
        h2: u8,
        h3: u8,
        w1: u8,
        w2: u8,
        w3: u8,
    };

    let mut a = [[0; N]; N];
    let mut ans = 0;
    dfs(&mut a, 0, &[h1, h2, h3], &[w1, w2, w3], &mut ans);
    println!("{}", ans);
}
