use proconio::input;

use crate::doubling::{Doubling, Transition, Value};

fn main() {
    input! {
        m: usize,
        a: usize,
        b: usize,
    };

    let pack = |(cur, next): (usize, usize)| {
        assert!(cur < m);
        assert!(next < m);
        cur * m + next
    };
    let unpack = |state: usize| {
        assert!(state < m * m);
        let cur = state / m;
        let next = state % m;
        (cur, next)
    };

    let inf = m * m;
    let db = Doubling::new(m * m, inf, |state| {
        let (prev, cur) = unpack(state);
        let next = (cur, (a * cur + b * prev) % m);
        Transition::new(pack(next), prev == 0 || cur == 0)
    });

    let mut ans = 0;
    for x in 0..m {
        for y in 0..m {
            let acc = db.fold(pack((x, y)), inf, false, |acc, t| acc || t.value);
            if !acc {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

impl Value for bool {
    fn op(&self, other: &Self) -> Self {
        *self || *other
    }
}
// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod doubling {
    #[derive(Debug, Clone)]
    pub struct Doubling<V> {
        transitions: Vec<Transition<V>>,
        n_state: usize,
        max_steps: usize,
        log2_max_steps: usize,
    }

    #[derive(Debug, Clone)]
    pub struct Transition<V> {
        pub next: usize,
        pub value: V,
    }

    impl<V> Transition<V> {
        pub fn new(next: usize, value: V) -> Self {
            Self { next, value }
        }
    }

    pub trait Value {
        fn op(&self, other: &Self) -> Self;
    }

    impl Value for () {
        fn op(&self, _other: &Self) -> Self {}
    }

    impl<V> Doubling<V>
    where
        V: Value,
    {
        pub fn new<F>(n_state: usize, max_steps: usize, step1: F) -> Self
        where
            F: Fn(usize) -> Transition<V>,
        {
            assert!(max_steps > 0);

            let log2_max_steps = max_steps.ilog2() as usize;

            let mut transitions = Vec::with_capacity(n_state * (log2_max_steps + 1));
            for i in 0..n_state {
                let t = step1(i);

                assert!(t.next < n_state);

                transitions.push(t);
            }

            for k in 1..=log2_max_steps {
                let offset = n_state * (k - 1);
                for i in 0..n_state {
                    let t1 = &transitions[offset + i];
                    let t2 = &transitions[offset + t1.next];
                    transitions.push(Transition {
                        next: t2.next,
                        value: t1.value.op(&t2.value),
                    });
                }
            }

            Self {
                transitions,
                n_state,
                max_steps,
                log2_max_steps,
            }
        }

        pub fn get(&self, start: usize, k: usize) -> &Transition<V> {
            assert!(start < self.n_state);
            assert!(k <= self.log2_max_steps);

            let offset = self.n_state * k;
            &self.transitions[offset + start]
        }

        pub fn fold<A, F>(&self, start: usize, step: usize, init: A, mut f: F) -> A
        where
            F: FnMut(A, &Transition<V>) -> A,
        {
            assert!(start < self.n_state);
            assert!(step <= self.max_steps);

            let mut i = start;
            let mut acc = init;
            for k in 0..=self.log2_max_steps {
                if step >> k & 1 == 1 {
                    let offset = self.n_state * k;
                    let t = &self.transitions[offset + i];
                    (i, acc) = (t.next, f(acc, t));
                }
            }

            acc
        }
    }
}
