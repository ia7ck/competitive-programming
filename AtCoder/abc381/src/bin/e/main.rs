use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
    };

    let mut delim = Vec::new();
    let mut one = Vec::new();
    let mut two = Vec::new();
    for i in 0..n {
        if s[i] == '/' {
            delim.push(i);
        }
        if s[i] == '1' {
            one.push(i);
        }
        if s[i] == '2' {
            two.push(i);
        }
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
        };

        // !s[l..=r].contains(&'/')
        let p = delim.partition_point(|&i| i <= r);
        if p == 0 || delim[p - 1] < l {
            println!("0");
            continue;
        }

        // k + 1 + k
        let f = |k: usize| -> bool {
            assert!(k >= 1);

            let start1 = one.partition_point(|&i| i < l);
            let end1 = start1 + k - 1;
            if end1 >= one.len() {
                return false;
            }

            let del = delim.partition_point(|&i| i < one[end1]);
            if del >= delim.len() {
                return false;
            }

            let start2 = two.partition_point(|&i| i < delim[del]);
            let end2 = start2 + k - 1;
            end2 < two.len() && two[end2] <= r
        };

        let mut ok = 0;
        let mut ng = n;
        while ng - ok > 1 {
            let mid = (ng + ok) / 2;
            if f(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        println!("{}", ok * 2 + 1);
    }
}
