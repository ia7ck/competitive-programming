use proconio::input;

fn f(n: usize, m: u8, buf: &mut Vec<u8>, ans: &mut Vec<Vec<u8>>) {
    if buf.len() == n {
        ans.push(buf.clone());
        return;
    }
    let last = buf.last().copied().unwrap_or(0);
    for x in (last + 1)..=m {
        buf.push(x);
        f(n, m, buf, ans);
        buf.pop();
    }
}

fn main() {
    input! {
        n: usize,
        m: u8,
    };

    let mut buf = Vec::new();
    let mut ans = Vec::new();
    f(n, m, &mut buf, &mut ans);
    ans.sort();
    for ans in ans {
        println!(
            "{}",
            ans.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
