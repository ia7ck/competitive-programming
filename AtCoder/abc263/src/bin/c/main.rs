use join::Join;
use proconio::input;

fn dfs(n: usize, m: usize, a: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if n == 0 {
        result.push(a.clone());
        return;
    }
    let lb = a.last().copied().unwrap_or(0);
    for x in (lb + 1)..=m {
        a.push(x);
        dfs(n - 1, m, a, result);
        a.pop();
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut a = Vec::new();
    let mut ans = Vec::new();
    dfs(n, m, &mut a, &mut ans);
    ans.sort();
    for ans in ans {
        println!("{}", ans.iter().join(" "));
    }
}
