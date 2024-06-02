use proconio::input_interactive;

fn main() {
    input_interactive! {
        n: usize,
    };

    let a = (1..=n).collect::<Vec<_>>();
    let mut a = merge_sort(&a); 

    loop {
        let min = a.remove(0);
        let max = a.pop().unwrap();
        println!("+ {} {}", min, max);
        input_interactive! {
            q: isize,
        };
        assert_ne!(q, -1);
        if a.is_empty() {
            break;
        }
        let q = q as usize;
        let i = {
            println!("? {} {}", q, a[0]);
            input_interactive! {
                qq: i8,
            };
            assert_ne!(qq, -1);
            if qq == 1 {
                0
            } else {
                let mut ok = 0;
                let mut ng = a.len();
                // a[ok] <= q 
                while ng - ok > 1 {
                    let mid = (ng + ok) / 2;
                    println!("? {} {}", q, a[mid]);
                    input_interactive! {
                        qq: i8,
                    };
                    assert_ne!(qq, -1);
                    if qq == 0 {
                        // a[mid] <= q
                        ok = mid;
                    } else {
                        ng = mid;
                    }
                }
                ok + 1
            }
        };
        a.insert(i, q);
    }

    println!("!");
}

fn merge_sort(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    if n <= 1 {
        return a.to_vec();
    }

    let left = merge_sort(&a[..n / 2]);
    let right = merge_sort(&a[n / 2..]);
    let mut res = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        println!("? {} {}", left[i], right[j]);
        input_interactive! {
            q: i8,
        };
        assert_ne!(q, -1);
        if q == 1 {
            res.push(left[i]);
            i += 1;
        } else {
            res.push(right[j]);
            j += 1;
        }
    }
    while i < left.len() {
        res.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        res.push(right[j]);
        j += 1;
    }
    res
}
