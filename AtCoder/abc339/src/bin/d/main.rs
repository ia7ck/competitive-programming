use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();
    {
        let mut players = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if s[i][j] == 'P' {
                    players.push((i, j));
                }
            }
        }
        assert_eq!(players.len(), 2);
        let (i1, j1) = players[0];
        let (i2, j2) = players[1];
        dist.insert(((i1, j1), (i2, j2)), 0);
        queue.push_back(((i1, j1), (i2, j2), 0));
    }

    while let Some((p1, p2, d)) = queue.pop_front() {
        if p1 == p2 {
            println!("{}", d);
            return;
        }
        let left = |(i, j): (usize, usize)| {
            if j >= 1 && s[i][j - 1] != '#' {
                Some((i, j - 1))
            } else {
                None
            }
        };
        let right = |(i, j): (usize, usize)| {
            if j + 1 < n && s[i][j + 1] != '#' {
                Some((i, j + 1))
            } else {
                None
            }
        };
        let up = |(i, j): (usize, usize)| {
            if i >= 1 && s[i - 1][j] != '#' {
                Some((i - 1, j))
            } else {
                None
            }
        };
        let down = |(i, j): (usize, usize)| {
            if i + 1 < n && s[i + 1][j] != '#' {
                Some((i + 1, j))
            } else {
                None
            }
        };

        let mut push2 = |p1: (usize, usize), p2: (usize, usize), d: usize| {
            if dist.contains_key(&(p1, p2)) == false {
                dist.insert((p1, p2), d + 1);
                queue.push_back((p1, p2, d + 1));
            }
        };

        macro_rules! push {
            ($dir: expr, $p1: expr, $p2: expr) => {
                match ($dir($p1), $dir($p2)) {
                    (Some(new_p1), Some(new_p2)) => {
                        push2(new_p1, new_p2, d);
                    }
                    (Some(new_p1), None) => {
                        push2(new_p1, $p2, d);
                    }
                    (None, Some(new_p2)) => {
                        push2($p1, new_p2, d);
                    }
                    _ => {}
                }
            };
        }

        push!(left, p1, p2);
        push!(right, p1, p2);
        push!(up, p1, p2);
        push!(down, p1, p2);
    }

    println!("-1");
}
