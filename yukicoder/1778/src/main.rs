use input_i_scanner::InputIScanner;
use lowest_common_ancestor::LowestCommonAncestor;
use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    macro_rules! scan {
        (($($t: ty),+)) => {
            ($(scan!($t)),+)
        };
        ($t: ty) => {
            _i_i.scan::<$t>() as $t
        };
        (($($t: ty),+); $n: expr) => {
            std::iter::repeat_with(|| scan!(($($t),+))).take($n).collect::<Vec<_>>()
        };
        ($t: ty; $n: expr) => {
            std::iter::repeat_with(|| scan!($t)).take($n).collect::<Vec<_>>()
        };
    }

    let (n, q) = scan!((usize, usize));
    let s = scan!(String);
    let s: Vec<char> = std::iter::once('^').chain(s.chars()).collect();

    let mut stack = Vec::new();
    stack.push(0);
    let mut parent = vec![0; n + 1];
    let mut lr_pos = HashMap::new();
    let mut rl_pos = HashMap::new();
    for i in 1..=n {
        if s[i] == '(' {
            stack.push(i);
        } else {
            let j = stack.pop().unwrap();
            let opt = lr_pos.insert(j, i);
            assert!(opt.is_none());
            let opt = rl_pos.insert(i, j);
            assert!(opt.is_none());
            parent[j] = stack.last().copied().unwrap();
        }
    }
    assert_eq!(stack, vec![0]);
    let mut edges = Vec::new();
    for i in 1..=n {
        if s[i] == '(' {
            edges.push((parent[i], i));
            edges.push((parent[i], lr_pos[&i]));
        }
    }
    let lca = LowestCommonAncestor::new(n + 1, edges.iter().copied());

    for _ in 0..q {
        let (x, y) = scan!((usize, usize));
        let x = if s[x] == '(' { x } else { rl_pos[&x] };
        let y = if s[y] == '(' { y } else { rl_pos[&y] };
        let ans_l = lca.get(x, y);
        if ans_l == 0 {
            println!("-1");
        } else {
            println!("{} {}", ans_l, lr_pos[&ans_l]);
        }
    }
}
