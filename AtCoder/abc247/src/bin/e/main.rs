use proconio::input;

fn f(a: &[u32], x: u32, y: u32) -> usize {
    for &a in a {
        assert!(y <= a && a <= x);
    }
    let mut last_x = 0;
    let mut last_y = 0;
    let mut ans = 0;
    for i in 0..a.len() {
        if a[i] == x {
            last_x = i + 1;
        }
        if a[i] == y {
            last_y = i + 1;
        }
        ans += last_x.min(last_y);
    }
    ans
}

fn main() {
    input! {
        n: usize,
        x: u32,
        y: u32,
        a: [u32; n],
    };
    let mut b = Vec::new();
    let mut c = Vec::new();
    for a in a {
        if y <= a && a <= x {
            c.push(a);
        } else {
            if c.len() >= 1 {
                b.push(c.drain(..).collect::<Vec<u32>>());
            }
        }
    }
    if c.len() >= 1 {
        b.push(c.drain(..).collect::<Vec<u32>>());
    }

    let mut ans = 0;
    for a in b {
        ans += f(&a, x, y);
    }
    println!("{}", ans);
}
