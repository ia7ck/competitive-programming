use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut x = Vec::new();
    let mut ix = Vec::new();
    for i in 0..s.len() {
        if s[i] != 'A' {
            x.push(s[i]);
            ix.push(i);
        }
    }
    let mut y = Vec::new();
    let mut iy = Vec::new();
    for i in 0..t.len() {
        if t[i] != 'A' {
            y.push(t[i]);
            iy.push(i);
        }
    }

    if x != y {
        println!("-1");
        return;
    }

    ix.insert(0, 0);
    ix.push(s.len() - 1);
    iy.insert(0, 0);
    iy.push(t.len() - 1);
    let mut ans = 0;
    for (wx, wy) in ix.windows(2).zip(iy.windows(2)) {
        let ax = wx[1] - wx[0];
        let ay = wy[1] - wy[0];
        ans += ax.abs_diff(ay);
    }

    println!("{}", ans);
}
