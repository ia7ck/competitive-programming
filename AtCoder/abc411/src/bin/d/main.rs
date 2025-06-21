use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n: usize,
        q: usize,
    };

    let mut queries = Vec::new();
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                p: Usize1,
            };
            queries.push(Query::Pull(p));
        } else if op == 2 {
            input! {
                p: Usize1,
                s: String,
            };
            queries.push(Query::Append(p, s));
        } else {
            input! {
                p: Usize1,
            };
            queries.push(Query::Push(p));
        }
    }

    let ans = solve(&queries, usize::MAX);
    println!("{}", ans);
}

enum Query {
    Pull(usize),
    Append(usize, String),
    Push(usize),
}

fn solve(queries: &[Query], target: usize) -> String {
    match queries.split_last() {
        None => String::new(),
        Some((last, queries)) => match last {
            &Query::Pull(p) => {
                if target == p {
                    solve(queries, usize::MAX)
                } else {
                    solve(queries, target)
                }
            }
            Query::Append(p, s) => {
                if target == *p {
                    let mut result = solve(queries, target);
                    result.push_str(s.as_str());
                    result
                } else {
                    solve(queries, target)
                }
            }
            &Query::Push(p) => {
                if target == usize::MAX {
                    solve(queries, p)
                } else {
                    solve(queries, target)
                }
            }
        },
    }
}
