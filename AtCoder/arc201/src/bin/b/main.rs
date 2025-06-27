use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            w: u64,
            items: [(u32, u64); n],
        };

        solve(n, w, items);
    }
}

fn solve(_n: usize, w: u64, items: Vec<(u32, u64)>) {
    let ans = f(w, items);
    println!("{}", ans);
}

fn f(w: u64, items: Vec<(u32, u64)>) -> u64 {
    if w == 0 {
        return 0;
    }

    if w % 2 == 0 {
        let (mut small, mut large): (Vec<_>, Vec<_>) =
            items.into_iter().partition(|&(x, _)| x == 0);

        small.sort_by_key(|&(_, y)| y);
        small.reverse();
        for pair in small.chunks(2) {
            let (_, y0) = pair[0];
            let y1 = pair.get(1).map(|&(_, y)| y).unwrap_or(0);
            large.push((1, y0 + y1));
        }

        for (x, _) in large.iter_mut() {
            *x -= 1;
        }

        f(w / 2, large)
    } else {
        let (mut small, mut large): (Vec<_>, Vec<_>) =
            items.into_iter().partition(|&(x, _)| x == 0);

        let mut ans = 0;

        small.sort_by_key(|&(_, y)| y);
        if let Some((_, y)) = small.pop() {
            ans += y;
        }

        large.append(&mut small);

        ans + f(w - 1, large)
    }
}
