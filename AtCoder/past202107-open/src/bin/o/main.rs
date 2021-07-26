use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..n {
        let x: u64 = rd.get();
        let y: u64 = rd.get();
        a.push(x);
        b.push(y);
    }

    let (a, b, s) = {
        let mut aa = vec![a[0]];
        let mut bb = vec![b[0]];
        let mut s = 0;
        for i in 1..n {
            if bb[bb.len() - 1] < b[i] {
                aa.push(a[i] + s);
                bb.push(b[i]);
                s = 0;
            } else {
                s += a[i];
            }
        }
        (aa, bb, s)
    };
    let n = a.len();
    let mut cum_sum = vec![0];
    for i in 0..n {
        cum_sum.push(cum_sum[i] + a[i]);
    }
    let inf = std::u64::MAX;
    let mut dp = vec![inf; n + 1];
    dp[0] = 0;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(dp[0]), a[0] - dp[0]));
    for i in 0..n {
        while let Some(&(_, y)) = heap.peek() {
            if y < b[i] {
                heap.pop();
            } else {
                break;
            }
        }
        match heap.peek() {
            Some(&(Reverse(x), _)) => {
                dp[i + 1] = x + b[i];
                if i + 2 <= n {
                    assert!(cum_sum[i + 2] >= dp[i + 1]);
                    heap.push((Reverse(dp[i + 1]), cum_sum[i + 2] - dp[i + 1]));
                }
            }
            None => {
                println!("-1");
                return;
            }
        }
    }
    assert!(dp[n] < inf);
    println!("{}", s + cum_sum[n] - dp[n]);
}
