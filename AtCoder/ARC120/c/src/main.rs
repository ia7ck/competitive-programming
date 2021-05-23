use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<u32> = rd.get_vec(n);
    let b: Vec<u32> = rd.get_vec(n);

    let a: Vec<u32> = a
        .into_iter()
        .enumerate()
        .map(|(i, x)| x + i as u32)
        .collect();
    let b: Vec<u32> = b
        .into_iter()
        .enumerate()
        .map(|(i, x)| x + i as u32)
        .collect();
    {
        let mut sorted_a = a.clone();
        sorted_a.sort();
        let mut sorted_b = b.clone();
        sorted_b.sort();
        if sorted_a != sorted_b {
            println!("-1");
            return;
        }
    }
    let mut ai: Vec<(u32, usize)> = a.iter().copied().enumerate().map(|(i, x)| (x, i)).collect();
    let mut bi: Vec<(u32, usize)> = b.iter().copied().enumerate().map(|(i, x)| (x, i)).collect();
    ai.sort();
    bi.sort();
    let mut perm = vec![0; n];
    for (&(_, i), &(_, j)) in ai.iter().zip(bi.iter()) {
        perm[i] = j;
    }
    let ans = inversion(&perm);
    println!("{}", ans);
}

fn inversion(a: &[usize]) -> usize {
    use fenwick_tree::FenwickTree;
    let n = a.len();
    for &x in a {
        assert!(x < n);
    }
    let mut res = 0;
    let mut ft = FenwickTree::new(n, 0);
    for &x in a {
        res += ft.sum((x + 1)..n);
        ft.add(x, 1);
    }
    res
}
