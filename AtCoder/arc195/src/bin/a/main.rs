use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    };

    let mut pos = Vec::new();
    let mut i = 0;
    for &b in &b {
        while i < n && a[i] != b {
            i += 1;
        }
        if i == n {
            break;
        }
        assert_eq!(a[i], b);
        pos.push(i);
        i += 1;
    }
    if pos.len() < m {
        println!("No");
        return;
    }

    let mut rev_pos = Vec::new();
    let mut i = n - 1;
    for &b in b.iter().rev() {
        while i != usize::MAX && a[i] != b {
            i = i.wrapping_sub(1);
        }
        if i == usize::MAX {
            break;
        }
        assert_eq!(a[i], b);
        rev_pos.push(i);
        i = i.wrapping_sub(1);
    }

    rev_pos.reverse();
    if pos == rev_pos {
        println!("No");
    } else {
        println!("Yes");
    }
}
