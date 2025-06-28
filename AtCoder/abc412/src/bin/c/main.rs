use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            s: [u32; n],
        };

        solve(n, s);
    }
}

fn solve(n: usize, s: Vec<u32>) {
    let mut a = s[1..(n - 1)].to_vec();
    a.sort_unstable();
    a.retain(|&x| x > s[0]);

    let mut ans = 1;
    let mut prev = s[0];
    while prev * 2 < s[n - 1] && !a.is_empty() {
        let p = a.partition_point(|&x| prev * 2 >= x);
        if p == 0 {
            break;
        } else {
            ans += 1;
            prev = a[p - 1];
            a = a.split_off(p);
        }
    }
    if prev * 2 >= s[n - 1] {
        ans += 1;
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
