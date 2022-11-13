use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    };

    a.sort();
    let mut b = a.clone();
    b.extend(a.clone());
    // eprintln!("{:?}", b);
    let mut sums = Vec::new();
    let mut last = 0;
    for i in 0..n {
        if i < last {
            continue;
        }
        // eprintln!("last = {}, {:?}", last, &b[last..]);
        assert_eq!(i, last);
        let mut s = b[i];
        last += 1;
        while last < b.len()
            && last - i < n
            && (b[last - 1] == b[last] || (b[last - 1] + 1) % m == b[last])
        {
            s += b[last];
            last += 1;
        }
        sums.push(s);
    }

    let total = a.iter().sum::<usize>();
    let max = sums.iter().max().copied().unwrap();
    assert!(total >= max);
    let ans = total - max;
    println!("{}", ans);
}
