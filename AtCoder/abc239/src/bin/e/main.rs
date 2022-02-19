use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (n, q) = scan!((usize, usize));
    let xs = scan!(u32; n);
    let mut g = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let (a, b) = scan!((usize, usize));
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let vk = scan!((usize, usize); q);

    let mut top20s = vec![vec![]; n];
    dfs(0, 0, &g, &xs, &mut top20s);
    for (v, k) in vk {
        let ans = top20s[v - 1][k - 1];
        println!("{}", ans);
    }
}

fn dfs(i: usize, p: usize, g: &Vec<Vec<usize>>, xs: &Vec<u32>, top20s: &mut Vec<Vec<u32>>) {
    top20s[i].push(xs[i]);
    for &j in &g[i] {
        if j == p {
            continue;
        }
        dfs(j, i, g, xs, top20s);
        for y in top20s[j].clone() {
            top20s[i].push(y);
        }
        top20s[i].sort();
        top20s[i].reverse();
        top20s[i].truncate(20);
    }
}
