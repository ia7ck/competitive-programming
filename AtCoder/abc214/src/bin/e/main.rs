use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let n: usize = rd.get();
        let lr: Vec<(usize, usize)> = (0..n)
            .map(|_| {
                let l: usize = rd.get();
                let r: usize = rd.get();
                (l, r)
            })
            .collect();
        solve(n, lr);
    }
}

fn solve(_: usize, lr: Vec<(usize, usize)>) {
    use std::collections::BTreeMap;
    let mut start = BTreeMap::new();
    for (l, r) in lr {
        start.entry(l).or_insert(Vec::new()).push(r);
    }
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    let mut cur = 1;
    loop {
        while let Some(Reverse(r)) = heap.pop() {
            if cur > r {
                println!("No");
                return;
            }
            cur += 1;
            if let Some(v) = start.get(&cur) {
                for &v in v {
                    heap.push(Reverse(v));
                }
            }
        }
        if let Some((&nxt, v)) = start.range(cur..).next() {
            cur = nxt;
            for &v in v {
                heap.push(Reverse(v));
            }
        } else {
            break;
        }
    }
    println!("Yes");
}
