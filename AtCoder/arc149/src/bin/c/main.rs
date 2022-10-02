use proconio::input;
use std::collections::HashSet;

fn is_prime(x: usize) -> bool {
    for y in 2..x {
        if x % y == 0 {
            return false;
        }
    }
    true
}

fn dfs(count: usize, n: usize, a: &mut Vec<Vec<usize>>) -> bool {
    if count == n * n {
        return true;
    }
    let mut used = HashSet::new();
    for c in 0..count {
        let (i, j) = (c / n, c % n);
        used.insert(a[i][j]);
    }
    let (i, j) = (count / n, count % n);
    for x in 1..=(n * n) {
        if used.contains(&x) {
            continue;
        }
        if i >= 1 && is_prime(a[i - 1][j] + x) {
            continue;
        }
        if j >= 1 && is_prime(a[i][j - 1] + x) {
            continue;
        }
        a[i][j] = x;
        if dfs(count + 1, n, a) {
            return true;
        }
        a[i][j] = 0;
    }
    false
}

fn print(n: usize, a: &Vec<Vec<usize>>) {
    for i in 0..n {
        for j in 0..n {
            print!("{}", a[i][j]);
            if j + 1 < n {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    input! {
        n: usize,
    };

    if n <= 10 {
        let mut a = vec![vec![0; n]; n];
        let found = dfs(0, n, &mut a);
        assert!(found);
        print(n, &a);
        return;
    }

    let mut odd_by_3 = vec![vec![]; 3];
    let mut even_by_3 = vec![vec![]; 3];
    for x in 1..=n * n {
        if x % 2 == 1 {
            odd_by_3[x % 3].push(x);
        } else {
            even_by_3[x % 3].push(x);
        }
    }
    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let x = if let Some(x) = odd_by_3[2].pop() {
                x
            } else if let Some(x) = odd_by_3[1].pop() {
                x
            } else if let Some(x) = odd_by_3[0].pop() {
                x
            } else if let Some(x) = even_by_3[0].pop() {
                x
            } else if let Some(x) = even_by_3[1].pop() {
                x
            } else if let Some(x) = even_by_3[2].pop() {
                x
            } else {
                unreachable!();
            };
            a[i][j] = x;
        }
    }
    print(n, &a);
}
