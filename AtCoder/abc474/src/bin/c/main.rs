use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [usize; n],
        a: [usize; q],
    };

    let mut prev = vec![None; n + 1];
    let mut next = vec![None; n + 1];
    for i in 0..n {
        if i >= 1 {
            prev[p[i]] = Some(p[i - 1]);
        }
        if i + 1 < n {
            next[p[i]] = Some(p[i + 1]);
        }
    }
    let mut last = p[n - 1];
    for a in a {
        if a == last {
            continue;
        }

        let prev_a = prev[a];
        let next_a = next[a];
        if let Some(prev_a) = prev_a {
            next[prev_a] = next_a;
        }
        if let Some(next_a) = next_a {
            prev[next_a] = prev_a;
        }

        assert_eq!(next[last], None);
        next[last] = Some(a);
        prev[a] = Some(last);
        next[a] = None;
        last = a;
    }

    let mut ans = Vec::new();
    ans.push(last);
    while let Some(x) = prev[last] {
        ans.push(x);
        last = x;
    }
    ans.reverse();
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
