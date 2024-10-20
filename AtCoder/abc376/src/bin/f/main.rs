use proconio::{input, marker::Usize1};

macro_rules! chmin {
    ($a: expr, $b: expr) => {
        $a = $a.min($b);
    };
}

fn main() {
    input! {
        n: usize,
        q: usize,
        ht: [(char, Usize1); q],
    };

    const INF: usize = usize::MAX / 4;

    let step = |start: usize, goal: usize, ng: usize| -> Vec<(usize, usize)> {
        assert_ne!(start, ng);
        let mut result = Vec::new();
        if start <= goal {
            // ++
            if start < ng && ng <= goal {
                result.push(((goal + 1) % n, (goal - ng + 1) + (goal - start)));
            } else {
                // ng < start || goal < ng
                result.push((ng, goal - start));
            }

            // --
            if start < ng && ng < goal {
                result.push((ng, n - (goal - start)));
            } else {
                // ng < start || goal <= ng
                let push_ng_cost = if ng < start {
                    ng + (n - goal + 1)
                } else {
                    ng - goal + 1
                };
                result.push(((n + goal - 1) % n, push_ng_cost + (n - (goal - start))));
            }
        } else {
            // --
            if goal <= ng && ng < start {
                result.push(((n + goal - 1) % n, (ng - goal + 1) + (start - goal)));
            } else {
                // ng < goal || start <= ng
                result.push((ng, start - goal));
            }

            // ++
            if goal < ng && ng < start {
                result.push((ng, n - (start - goal)));
            } else {
                // ng <= goal || start <= ng
                let push_ng_cost = if ng <= goal {
                    goal - ng + 1
                } else {
                    (n - ng) + goal + 1
                };
                result.push(((goal + 1) % n, push_ng_cost + (n - (start - goal))));
            }
        }
        result
    };

    #[derive(Debug, Clone, Copy)]
    enum HandKind {
        L,
        R,
    }

    #[derive(Debug, Clone, Copy)]
    struct Hand {
        kind: HandKind,
        pos: usize,
    }
    impl Hand {
        fn new(kind: HandKind, pos: usize) -> Self {
            Self { kind, pos }
        }
    }

    #[derive(Debug)]
    struct S {
        hand: Hand,
        values: Vec<usize>, // the other hand
    }
    impl S {
        fn new(hand: Hand, values: Vec<usize>) -> Self {
            Self { hand, values }
        }
    }

    let mut dp = {
        let mut v = vec![INF; n];
        v[1] = 0;
        S::new(Hand::new(HandKind::L, 0), v)
    };
    for (h, t) in ht {
        use HandKind::*;

        let h = if h == 'L' { L } else { R };
        let mut new_v = vec![INF; n];
        for (i, &v) in dp.values.iter().enumerate() {
            let (start, goal, ng) = match (dp.hand, h) {
                (Hand { kind: L, pos }, L) => (pos, t, i), // i = right
                (Hand { kind: L, pos }, R) => (i, t, pos), // i = right
                (Hand { kind: R, pos }, L) => (i, t, pos), // i = left
                (Hand { kind: R, pos }, R) => (pos, t, i), // i = left
            };
            if start == ng {
                continue;
            }
            for (new_ng, cost) in step(start, goal, ng) {
                chmin!(new_v[new_ng], v + cost);
            }
        }
        dp = S::new(Hand::new(h, t), new_v);
    }

    let mut ans = INF;
    for v in dp.values {
        chmin!(ans, v);
    }
    assert_ne!(ans, INF);
    println!("{}", ans);
}
