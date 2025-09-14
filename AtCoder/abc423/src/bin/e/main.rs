use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        queries: [(usize, usize); q],
    };

    let mut b = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] = b[i] + a[i];
    }
    let mut c = vec![0; n + 1];
    for i in 0..n {
        c[i + 1] = c[i] + b[i + 1];
    }
    let mut d = vec![0; n + 1];
    for i in 0..n {
        d[i + 1] = d[i] + c[i + 1];
    }
    let mut xb = vec![0; n + 1];
    for i in 0..n {
        xb[i + 1] = xb[i] + (i + 1) * b[i];
    }

    for (l, r) in queries {
        let mut ans = 0;

        // O(n^2)
        // for i in l..=r {
        //     for j in i..=r {
        //         ans += b[j] - b[i - 1];
        //     }
        // }

        // O(n^2)
        // for i in l..=r {
        //     ans += b[i..=r].iter().sum::<usize>();
        //     ans -= (r - i + 1) * b[i - 1];
        // }

        ans += (r - l + 1) * c[r];
        ans -= d[r - 1] - if l - 1 == 0 { 0 } else { d[l - 1 - 1] };

        ans += xb[r] - xb[l - 1];
        ans -= (r + 1) * (c[r - 1] - if l - 1 == 0 { 0 } else { c[l - 1 - 1] });

        println!("{}", ans);
    }
}
