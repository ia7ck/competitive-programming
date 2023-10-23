use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut a = vec![VecDeque::new(); m];
    for i in 0..m {
        input! {
            k: usize,
            b: [Usize1; k],
        };
        a[i] = VecDeque::from(b);
    }

    let mut pos = vec![vec![]; n];
    for i in 0..m {
        for &x in &a[i] {
            pos[x].push(i);
        }
    }
    for i in 0..n {
        assert_eq!(pos[i].len(), 2);
    }
    let mut top = vec![0; n];
    for i in 0..m {
        let j = a[i].pop_front().unwrap();
        top[j] += 1;
    }
    let mut que = VecDeque::new();
    for i in 0..n {
        if top[i] == 2 {
            que.push_back(i);
        }
    }
    while let Some(i) = que.pop_front() {
        assert_eq!(top[i], 2);
        for &p in &pos[i] {
            if let Some(j) = a[p].pop_front() {
                top[j] += 1;
                if top[j] == 2 {
                    que.push_back(j);
                }
            }
        }
    }
    let mut ok = true;
    for i in 0..n {
        if top[i] != 2 {
            ok = false;
        }
    }
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
