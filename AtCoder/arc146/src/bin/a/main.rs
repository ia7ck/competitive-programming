use proconio::input;

fn len(mut x: u64) -> usize {
    let mut l = 0;
    while x > 0 {
        x /= 10;
        l += 1;
    }
    l
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    let mut b = a.clone();
    b.sort_by_key(|&x| len(x));
    b.reverse();
    let v = if len(b[0]) == len(b[1]) && len(b[1]) == len(b[2]) {
        let l = len(b[0]);
        let mut c: Vec<u64> = a.iter().copied().filter(|&x| len(x) == l).collect();
        c.sort();
        c.reverse();
        assert!(c.len() >= 3);
        vec![c[0], c[1], c[2]]
    } else if len(b[0]) == len(b[1]) && len(b[1]) > len(b[2]) {
        let l = len(b[2]);
        let mut c: Vec<u64> = a.iter().copied().filter(|&x| len(x) == l).collect();
        c.sort();
        c.reverse();
        assert!(c.len() >= 1);
        vec![b[0].max(b[1]), b[0].min(b[1]), c[0]]
    } else if len(b[0]) > len(b[1]) && len(b[1]) == len(b[2]) {
        let l = len(b[1]);
        let mut c: Vec<u64> = a.iter().copied().filter(|&x| len(x) == l).collect();
        c.sort();
        c.reverse();
        assert!(c.len() >= 2);
        vec![b[0], c[0], c[1]]
    } else if len(b[0]) > len(b[1]) && len(b[1]) > len(b[2]) {
        let l = len(b[2]);
        let mut c: Vec<u64> = a.iter().copied().filter(|&x| len(x) == l).collect();
        c.sort();
        c.reverse();
        assert!(c.len() >= 1);
        vec![b[0], b[1], c[0]]
    } else {
        unreachable!();
    };

    let mut ans = 0_u64;
    for &(i, j, k) in &[
        (0, 1, 2),
        (0, 2, 1),
        (1, 0, 2),
        (1, 2, 0),
        (2, 0, 1),
        (2, 1, 0),
    ] {
        let s = format!("{}{}{}", v[i], v[j], v[k]);
        ans = ans.max(s.parse::<u64>().unwrap());
    }
    println!("{}", ans);
}
