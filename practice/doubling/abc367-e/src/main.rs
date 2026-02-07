use proconio::{input, marker::Usize1};

use crate::doubling::{Doubling, Transition, Value};

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [Usize1; n],
        a: [u32; n],
    };

    struct NoOp;
    impl Value for NoOp {
        fn op(&self, _other: &Self) -> Self {
            NoOp {}
        }
    }

    let db = Doubling::new(n, k, |i| Transition::new(x[i], NoOp {}));
    let mut ans = Vec::new();
    // slow
    for i in 0..n {
        let acc = db.fold(i, k, i, |_, t| t.next);
        ans.push(a[acc]);
    }

    println!(
        "{}",
        ans.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

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

    impl<V> Doubling<V>
    where
        V: Value,
    {
        pub fn new<F>(n_state: usize, max_steps: usize, step1: F) -> Self
        where
            F: Fn(usize) -> Transition<V>,
        {
            let log2_max_steps = if max_steps == 0 {
                0
            } else {
                max_steps.ilog2() as usize
            };

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

        pub fn fold<A, F>(&self, start: usize, step: usize, init: A, f: F) -> A
        where
            F: Fn(A, &Transition<V>) -> A,
        {
            assert!(start < self.n_state);
            assert!(step <= self.max_steps);

            let mut i = start;
            let mut acc = init;
            for k in 0..=self.log2_max_steps {
                if step >> k & 1 == 1 {
                    let offset = self.n_state * k;
                    let t = &self.transitions[offset + i];
                    (i, acc) = (t.next, f(acc, &t));
                }
            }

            acc
        }
    }
}
