use proconio::{input, marker::Chars};
use suffix_array::suffix_array;

fn main() {
    input! {
        s: Chars,
        q: usize,
    };

    let sa = suffix_array(&s);
    for _ in 0..q {
        input! {
            t: Chars,
        };

        let p = sa.partition_point(|&i| &s[i..(i + t.len()).min(s.len())] < &t[..]);
        // let pp = {
        //     let mut tt = t.clone();
        //     tt.push('~'); // 'z' < '~'
        //     sa.partition_point(|&i| &s[i..(i + tt.len()).min(s.len())] < &tt[..])
        // };
        let pp = sa.partition_point(|&i| &s[i..(i + t.len()).min(s.len())] <= &t[..]);

        assert!(pp >= p);
        let ans = pp - p;
        println!("{}", ans);
    }
}
