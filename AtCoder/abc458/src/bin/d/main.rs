use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        x: u32,
        q: usize,
        queries: [(u32, u32); q],
    };

    let mut small = BinaryHeap::<u32>::new();
    let mut large = BinaryHeap::<Reverse<u32>>::new();
    let mut med = x;
    for (a, b) in queries {
        assert_eq!(small.len(), large.len());

        let (a, b) = (a.min(b), a.max(b));
        if a <= med && med <= b {
            small.push(a);
            large.push(Reverse(b));
        } else if med < a {
            small.push(med);
            large.push(Reverse(a));
            large.push(Reverse(b));
            let Reverse(lg) = large.pop().unwrap();
            med = lg;
        } else if b < med {
            large.push(Reverse(med));
            small.push(a);
            small.push(b);
            let sm = small.pop().unwrap();
            med = sm;
        } else {
            unreachable!()
        }

        println!("{}", med);

        assert!(small.peek() <= Some(&med));
        // assert!(!(Some(&Reverse(med)) <= large.peek()));
        {
            let &Reverse(lg) = large.peek().unwrap();
            assert!(med <= lg);
        }
        assert_eq!(small.len(), large.len());
    }
}
