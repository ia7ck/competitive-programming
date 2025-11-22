use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            x: [usize; n],
            y: [usize; m],
        };

        solve(n, m, x, y);
    }
}

fn solve(n: usize, m: usize, x: Vec<usize>, y: Vec<usize>) {
    if HashSet::<usize>::from_iter(x.clone()).len() < n {
        println!("No");
        return;
    }
    if HashSet::<usize>::from_iter(y.clone()).len() < m {
        println!("No");
        return;
    }

    let mut ub = vec![vec![usize::MAX; m]; n];
    for i in 0..n {
        for j in 0..m {
            ub[i][j] = ub[i][j].min(x[i]).min(y[j]);
        }
    }

    let mut xpos = vec![None; n * m + 1];
    let mut ypos = vec![None; n * m + 1];
    for i in 0..n {
        xpos[x[i]] = Some(i);
    }
    for j in 0..m {
        ypos[y[j]] = Some(j);
    }
    let mut put = vec![false; n * m + 1];
    let mut ans = vec![vec![0; m]; n];
    for a in 1..=(n * m) {
        match (xpos[a], ypos[a]) {
            (Some(xp), Some(yp)) => {
                assert_eq!(ans[xp][yp], 0);
                put[a] = true;
                ans[xp][yp] = a;
            }
            _ => continue,
        }
    }

    for a in 1..=(n * m) {
        if put[a] {
            continue;
        }
        match (xpos[a], ypos[a]) {
            (Some(xp), None) => {
                let Some(j) = (0..m).find(|&j| ans[xp][j] == 0 && a <= ub[xp][j]) else {
                    println!("No");
                    return;
                };
                put[a] = true;
                ans[xp][j] = a;
            }
            (None, Some(yp)) => {
                let Some(i) = (0..n).find(|&i| ans[i][yp] == 0 && a <= ub[i][yp]) else {
                    println!("No");
                    return;
                };
                put[a] = true;
                ans[i][yp] = a;
            }
            _ => continue,
        }
    }

    let rest = (1..=(n * m)).filter(|&a| !put[a]).collect::<Vec<_>>();
    let mut pos = Vec::new();
    for i in 0..n {
        for j in 0..m {
            if ans[i][j] == 0 {
                pos.push((i, j));
            }
        }
    }
    pos.sort_unstable_by_key(|&(i, j)| ub[i][j]);
    assert_eq!(rest.len(), pos.len());
    for (&a, &(i, j)) in rest.iter().zip(&pos) {
        if a > ub[i][j] {
            println!("No");
            return;
        }
        put[a] = true;
        ans[i][j] = a;
    }

    for i in 0..n {
        for j in 0..m {
            assert_ne!(ans[i][j], 0);
        }
    }

    if !check(n, m, &x, &y, &ans) {
        println!("No");
        return;
    }

    println!("Yes");
    for row in ans {
        println!(
            "{}",
            row.iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

fn check(n: usize, m: usize, x: &Vec<usize>, y: &Vec<usize>, ans: &Vec<Vec<usize>>) -> bool {
    for i in 0..n {
        let a = ans[i].iter().max().copied().unwrap();
        if a != x[i] {
            return false;
        }
    }
    for j in 0..m {
        let a = (0..n).map(|i| ans[i][j]).max().unwrap();
        if a != y[j] {
            return false;
        }
    }

    true
}
