use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut ans = Vec::new();
    for first in 1..=m {
        let mut a = vec![first];
        dfs(n, m, &mut a, &mut ans);
    }
    ans.sort();

    println!("{}", ans.len());
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

fn dfs(n: usize, m: usize, a: &mut Vec<usize>, acc: &mut Vec<Vec<usize>>) {
    if a.len() == n {
        acc.push(a.clone());
        return;
    }
    let prev = a.last().copied().unwrap();
    for x in (prev + 10)..=m {
        let last = x + 10 * (n - a.len() - 1);
        if last > m {
            continue;
        }
        a.push(x);
        dfs(n, m, a, acc);
        a.pop();
    }
}
