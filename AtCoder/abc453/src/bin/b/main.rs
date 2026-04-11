use proconio::input;

fn main() {
    input! {
        t: usize,
        x: u32,
        a: [u32; t + 1],
    };

    let mut ans = vec![(0, a[0])];
    for t in 1..=t {
        let (_, last) = ans.last().copied().unwrap();
        if a[t].abs_diff(last) >= x {
            ans.push((t, a[t]));
        }
    }

    for (t, a) in ans {
        println!("{} {}", t, a);
    }
}
