use proconio::{input, marker::Chars};

fn check(s: &Vec<char>, t: &Vec<char>) -> bool {
    assert_eq!(s.len(), t.len());
    let n = s.len();
    for i in 0..n {
        assert!(s[i] == 'A' || s[i] == 'B' || s[i] == 'C');
        assert!(t[i] == 'A' || t[i] == 'B');
    }

    let (mut a_s, mut b_s) = (0, 0);
    let (mut a_t, mut b_t) = (0, 0);
    for i in 0..n {
        if s[i] == 'A' {
            a_s += 1;
        } else if s[i] == 'B' {
            b_s += 1;
        }
        if t[i] == 'A' {
            a_t += 1;
        } else if t[i] == 'B' {
            b_t += 1;
        }
    }

    if a_s > a_t || b_s > b_t {
        return false;
    }

    let mut a = a_t - a_s;
    let mut s = s.clone();
    for i in 0..n {
        if s[i] == 'C' {
            if a > 0 {
                s[i] = 'A';
                a -= 1;
            } else {
                s[i] = 'B';
            }
        }
    }

    assert_eq!(a, 0);

    {
        let a_s = s.iter().filter(|&&ch| ch == 'A').count();
        let a_t = t.iter().filter(|&&ch| ch == 'A').count();
        assert_eq!(a_s, a_t);
        let b_s = s.iter().filter(|&&ch| ch == 'B').count();
        let b_t = t.iter().filter(|&&ch| ch == 'B').count();
        assert_eq!(b_s, b_t);
    }

    let mut pos_s = Vec::new();
    let mut pos_t = Vec::new();
    for i in 0..n {
        if s[i] == 'A' {
            pos_s.push(i);
        }
        if t[i] == 'A' {
            pos_t.push(i);
        }
    }
    
    for i in 0..pos_s.len() {
        if pos_s[i] > pos_t[i] {
            return false;
        }
    }
    true
}

fn solve() {
    input! {
        n: usize,
        x: Chars,
        y: Chars,
    };
    // eprintln!("n = {}, x = {:?}, y = {:?}", n, x, y);

    let (mut c_x, mut c_y) = (0, 0);
    for i in 0..n {
        if x[i] == 'C' {
            c_x += 1;
        }
        if y[i] == 'C' {
            c_y += 1;
        }
    }
    if c_x < c_y {
        println!("No");
        return;
    }

    let mut s = Vec::new();
    let mut t = Vec::new();
    for i in 0..n {
        if y[i] == 'C' {
            if x[i] != 'C' {
                println!("No");
                return;
            }
            if check(&s, &t) == false {
                println!("No");
                return;
            }
            s.clear();
            t.clear();
        } else {
            s.push(x[i]);
            t.push(y[i]);
        }
    }
    if check(&s, &t) == false {
        println!("No");
        return;
    }

    println!("Yes");
}

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        solve();
    }
}
