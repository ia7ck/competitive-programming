use proconio::input;

fn main() {
    input! {
        l: u64,
        r: u64,
    };

    let ans = f(r) - f(l - 1);
    println!("{}", ans);
}

fn f(r: u64) -> u64 {
    if r < 10 {
        return 0;
    }

    let mut ans = 0;
    let r = r
        .to_string()
        .bytes()
        .map(|d| u64::from(d - b'0'))
        .collect::<Vec<_>>();
    if r[1..].iter().all(|&d| r[0] > d) {
        ans += 1;
    }

    for i in 1..r.len() {
        for _ in 0..r[i].min(r[0]) {
            ans += r[0].pow((r.len() - i - 1) as u32);
        }
        if r[i] >= r[0] {
            break;
        }
    }

    for d in 1..r[0] {
        ans += d.pow((r.len() - 1) as u32);
    }


    for i in 1..(r.len() - 1) {
        for d in 1_u64..=9 {
            ans += d.pow((r.len() - i - 1) as u32);
        }
    }

    ans
}

#[allow(unused)]
fn g(r: u64) -> u64 {
    let mut ans = 0;
    for x in 10..=r {
        let x = x.to_string().chars().collect::<Vec<_>>();
        if x[1..].iter().all(|&d| x[0] > d) {
            ans += 1;
        }
    }
    ans
}
