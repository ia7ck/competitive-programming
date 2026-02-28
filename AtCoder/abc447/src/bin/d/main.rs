use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };

    let mut qa = VecDeque::new();
    let mut qb = VecDeque::new();
    let mut qc = VecDeque::new();
    for i in 0..s.len() {
        match s[i] {
            'A' => qa.push_back(i),
            'B' => qb.push_back(i),
            'C' => qc.push_back(i),
            _ => unreachable!(),
        }
    }

    let mut ans = 0;
    while let Some(ia) = qa.pop_front() {
        let Some(ib) = pop_front_until(&mut qb, |ib| ia < ib) else {
            break;
        };
        let Some(_ic) = pop_front_until(&mut qc, |ic| ib < ic) else {
            break;
        };
        ans += 1;
    }

    println!("{}", ans);
}

fn pop_front_until(q: &mut VecDeque<usize>, pred: impl Fn(usize) -> bool) -> Option<usize> {
    while let Some(x) = q.pop_front() {
        if pred(x) {
            return Some(x);
        }
    }
    None
}
