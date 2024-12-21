use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        sx: i64,
        sy: i64,
        xy: [(i64, i64); n],
        dc: [(char, i64); m],
    };

    let mut tate = HashMap::new();
    let mut yoko = HashMap::new();
    let (mut x, mut y) = (sx, sy);
    for (d, c) in dc {
        match d {
            'U' => {
                tate.entry(x).or_insert_with(Vec::new).push((y, y + c));
                y += c;
            }
            'D' => {
                tate.entry(x).or_insert_with(Vec::new).push((y - c, y));
                y -= c;
            }
            'L' => {
                yoko.entry(y).or_insert_with(Vec::new).push((x - c, x));
                x -= c;
            }
            'R' => {
                yoko.entry(y).or_insert_with(Vec::new).push((x, x + c));
                x += c;
            }
            _ => unreachable!(),
        }
    }

    let mut y_by_x = HashMap::new();
    let mut x_by_y = HashMap::new();
    for (i, &(x, y)) in xy.iter().enumerate() {
        y_by_x.entry(x).or_insert_with(Vec::new).push((y, i));
        x_by_y.entry(y).or_insert_with(Vec::new).push((x, i));
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Event {
        In(i64),
        Point(i64, usize),
        Out(i64),
    }

    impl Event {
        fn value(&self) -> (i64, i8) {
            match self {
                Event::In(v) => (*v, -1),
                Event::Point(v, _) => (*v, 0),
                Event::Out(v) => (*v, 1),
            }
        }
    }

    impl PartialOrd for Event {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Event {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.value().cmp(&other.value())
        }
    }

    let mut ans = vec![false; n];
    for (x, ys) in y_by_x {
        let mut events = Vec::new();
        for (y, i) in ys {
            events.push(Event::Point(y, i));
        }
        if let Some(ys) = tate.get(&x) {
            for &(l, r) in ys {
                events.push(Event::In(l));
                events.push(Event::Out(r));
            }
        }
        events.sort_unstable();
        let mut cover = 0;
        for e in events {
            match e {
                Event::In(_) => cover += 1,
                Event::Point(_, i) => ans[i] |= cover > 0,
                Event::Out(_) => cover -= 1,
            }
        }
    }

    for (y, xs) in x_by_y {
        let mut events = Vec::new();
        for (x, i) in xs {
            events.push(Event::Point(x, i));
        }
        if let Some(xs) = yoko.get(&y) {
            for &(l, r) in xs {
                events.push(Event::In(l));
                events.push(Event::Out(r));
            }
        }
        events.sort_unstable();
        let mut cover = 0;
        for e in events {
            match e {
                Event::In(_) => cover += 1,
                Event::Point(_, i) => ans[i] |= cover > 0,
                Event::Out(_) => cover -= 1,
            }
        }
    }

    let count = ans.iter().filter(|&&b| b).count();
    println!("{} {} {}", x, y, count);
}
