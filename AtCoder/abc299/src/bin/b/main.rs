use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u32,
        c: [u32; n],
        r: [u32; n],
    };

    let ans = if c.contains(&t) {
        let a = (0..n).filter(|&i| c[i] == t).collect::<Vec<_>>();
        let rmax = a.iter().map(|&i| r[i]).max().unwrap();
        r.iter().position(|&r| r == rmax).unwrap()
    } else {
        let mut ans = 0;
        for i in 1..n {
            if c[i] == c[0] && r[i] > r[ans] {
                ans = i;
            }
        }
        ans
    };

    println!("{}", ans + 1);
}
