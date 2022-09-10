use proconio::input;

fn calc(s: &[char]) -> u64 {
    let mut x = 0;
    let mut f = [0_u64; 10];
    let mut total = 0;
    for &ch in s {
        if ch == 'X' {
            x += 1;
        } else {
            let d = ch as usize - '0' as usize;
            f[d] += 1;
            total += (d as u64) * x;
        }
    }
    total
}

fn main() {
    input! {
        n: usize,
        mut ss: [String; n],
    };

    ss.sort_by(|s, t| {
        let mut st = Vec::new();
        for ch in s.chars().chain(t.chars()) {
            st.push(ch);
        }
        let mut ts = Vec::new();
        for ch in t.chars().chain(s.chars()) {
            ts.push(ch);
        }
        calc(&ts).cmp(&calc(&st)) // tie-break ??
    });

    let mut ans = Vec::new();
    for s in ss {
        for ch in s.chars() {
            ans.push(ch);
        }
    }
    println!("{}", calc(&ans));
}
