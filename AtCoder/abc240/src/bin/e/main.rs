use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let n = scan!(usize);
    let mut g = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let (a, b) = scan!((usize, usize));
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    let mut ans = vec![(0, 0); n];
    dfs(0, 0, &g, 1, &mut ans);
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}

fn dfs(
    i: usize,
    p: usize,
    g: &Vec<Vec<usize>>,
    start: usize,
    ans: &mut Vec<(usize, usize)>,
) -> usize {
    let leaf = g[i].iter().filter(|&&j| j != p).count() == 0;
    if leaf {
        ans[i] = (start, start);
        return start;
    }

    let mut end = start;
    for &j in &g[i] {
        if j == p {
            continue;
        }
        let child_end = dfs(j, i, g, end, ans);
        end = child_end + 1;
    }
    ans[i] = (start, end - 1);
    end - 1
}
