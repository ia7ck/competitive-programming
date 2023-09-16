use proconio::{input, marker::Usize1};

fn visit(i: usize, g: &Vec<Vec<(usize, i64, i64)>>, ans: &mut Vec<Option<(i64, i64)>>) {
    let Some((x, y)) = ans[i] else {
        return;
    };
    for &(j, dx, dy) in &g[i] {
        match ans[j] {
            None => {
                ans[j] = Some((x + dx, y + dy));
                visit(j, g, ans);
            }
            Some((xx, yy)) => {
                assert_eq!(xx, x + dx);
                assert_eq!(yy, y + dy);
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abxy: [(Usize1, Usize1, i64, i64); m],
    };

    let mut g = vec![vec![]; n];
    for (a, b, x, y) in abxy {
        g[a].push((b, x, y));
        g[b].push((a, -x, -y));
    }

    let mut ans = vec![None; n];
    ans[0] = Some((0, 0));
    visit(0, &g, &mut ans);
    for ans in ans {
        if let Some((x, y)) = ans {
            println!("{} {}", x, y);
        } else {
            println!("undecidable");
        }
    }
}
