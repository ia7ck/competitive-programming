use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize; m],
    };

    let p = (d.iter().sum::<usize>() + 1) / 2;
    for a in 0..m {
        let s1 = d[..a].iter().sum::<usize>();
        let s2 = d[..(a + 1)].iter().sum::<usize>();
        if s1 < p && p <= s2 {
            let b = p - s1;
            println!("{} {}", a + 1, b);
            return;
        }
    }

    unreachable!();
}
