use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, usize); q],
    };

    let mut to = vec![0; n * 2 + 1];
    let mut from = vec![0; n * 2 + 1];
    for i in 1..=n {
        to[i + n] = i;
        from[i] = i + n;
    }

    for (c, p) in queries {
        assert_eq!(to[p], 0);

        to[from[c]] = 0;
        to[p] = c;
        from[c] = p;
    }

    let mut ans = Vec::new();
    for i in 1..=n {
        let mut count = 0;
        let mut v = i + n;
        loop {
            v = to[v];
            if v == 0 {
                break;
            }
            count += 1;
        }
        ans.push(count);
    }

    println!(
        "{}",
        ans.iter()
            .map(|ans| ans.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
