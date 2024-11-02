use proconio::input;
use segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };

    let mut count = SegmentTree::new(m, 0, |x, y| x + y);
    let mut value = SegmentTree::new(m, 0, |x, y| x + y);
    let mut ans = 0_usize;
    let mut p_sum = 0;
    count.update(0, 1);
    for &x in &a {
        p_sum += x;
        p_sum %= m;
        let small = count.fold(0..p_sum);
        let small_sum = value.fold(0..p_sum);
        let large = count.fold((p_sum + 1)..);
        let large_sum = value.fold((p_sum + 1)..);
        assert!(p_sum * small >= small_sum);
        ans += p_sum * small - small_sum;
        assert!((p_sum + m) * large >= large_sum);
        ans += (p_sum + m) * large - large_sum;
        count.update(p_sum, count.get(p_sum) + 1);
        value.update(p_sum, value.get(p_sum) + p_sum);
    }

    println!("{}", ans);

    if cfg!(debug_assertions) {
        let mut count = vec![0; m];
        let mut value = vec![0; m];
        let mut res = 0;
        let mut p_sum = 0;
        count[0] += 1;
        for &x in &a {
            p_sum += x;
            p_sum %= m;
            let small = count[..p_sum].iter().sum::<usize>();
            let small_sum = value[..p_sum].iter().sum::<usize>();
            let large = count[(p_sum + 1)..].iter().sum::<usize>();
            let large_sum = value[(p_sum + 1)..].iter().sum::<usize>();
            assert!(p_sum * small >= small_sum);
            res += p_sum * small - small_sum;
            assert!((p_sum + m) * large >= large_sum);
            res += (p_sum + m) * large - large_sum;
            count[p_sum] += 1;
            value[p_sum] += p_sum;
            eprintln!("small = {small}, small_sum = {small_sum}, large = {large}, large_sum = {large_sum}");
            eprintln!("p_sum = {p_sum}, count = {count:?}, res = {res}");
        }
        assert_eq!(res, ans);
    }
}
