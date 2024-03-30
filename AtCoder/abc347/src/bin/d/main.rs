use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: u64,
    };

    if let Some((x, y)) = solve(a, b, c) {
        assert!(x < (1_u64 << 60));
        assert!(y < (1_u64 << 60));
        assert!(x.count_ones() == a as u32);
        assert!(y.count_ones() == b as u32);
        assert!(x ^ y == c, "x = {x}, y = {y}, c = {c}");
        println!("{} {}", x, y);
    } else {
        println!("-1");
    }
}

fn solve(a: usize, b: usize, c: u64) -> Option<(u64, u64)> {
    if a < b {
        let (x, y) = solve(b, a, c)?;
        return Some((y, x));
    }

    let (mut x, mut y) = (0_u64, 0_u64);
    let one = (0..60).filter(|i| c >> i & 1 == 1).collect::<Vec<_>>();
    if a - b > one.len() {
        return None;
    }
    for i in &one[0..(a - b)] {
        x ^= 1 << i;
    }
    // a = b
    for chunk in one[(a - b)..].chunks_exact(2) {
        let (i, j) = (chunk[0], chunk[1]);
        if y.count_ones() < b as u32 {
            x ^= 1 << i;
            y ^= 1 << j;
        }
    }
    for i in 0..60 {
        if c >> i & 1 == 0 {
            if y.count_ones() < b as u32 {
                x ^= 1 << i;
                y ^= 1 << i;
            }
        }
    }

    if x.count_ones() == a as u32 && y.count_ones() == b as u32 && x ^ y == c {
        Some((x, y))
    } else {
        None
    }
}
