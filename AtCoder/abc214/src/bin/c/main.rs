use procon_reader::ProconReader;
use std::cmp::Reverse;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let s: Vec<u64> = rd.get_vec(n);
    let t: Vec<u64> = rd.get_vec(n);

    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push(Reverse((t[i], i)));
    }
    let mut ans = vec![0; n];
    while let Some(Reverse((t, i))) = heap.pop() {
        if ans[i] >= 1 {
            continue;
        }
        ans[i] = t;
        heap.push(Reverse((t + s[i], (i + 1) % n)));
    }
    for ans in ans {
        assert!(ans >= 1);
        println!("{}", ans);
    }
}
