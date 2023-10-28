use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Chars};

fn search(n: usize, row: &Vec<char>, col: &Vec<char>, pos: usize, s: &mut Vec<Vec<char>>) -> bool {
    if pos == n * n {
        for i in 0..n {
            if HashSet::from([&'A', &'B', &'C']).is_subset(&HashSet::from_iter(&s[i])) == false {
                return false;
            }
        }
        for j in 0..n {
            let mut t = Vec::new();
            for i in 0..n {
                t.push(s[i][j]);
            }
            if HashSet::from([&'A', &'B', &'C']).is_subset(&HashSet::from_iter(&t)) == false {
                return false;
            }
        }
        return true;
    }
    let (i, j) = (pos / n, pos % n);
    let mut left = HashMap::new();
    for k in 0..j {
        if s[i][k] == '.' {
            continue;
        };
        *left.entry(s[i][k]).or_insert(0) += 1;
    }
    let mut up = HashMap::new();
    for k in 0..i {
        if s[k][j] == '.' {
            continue;
        };
        *up.entry(s[k][j]).or_insert(0) += 1;
    }
    let x = if left.is_empty() {
        HashSet::from([row[i]])
    } else {
        let mut set = HashSet::new();
        for c in ['A', 'B', 'C'] {
            if left.contains_key(&c) == false {
                set.insert(c);
            }
        }
        set
    };
    let y = if up.is_empty() {
        HashSet::from([col[j]])
    } else {
        let mut set = HashSet::new();
        for c in ['A', 'B', 'C'] {
            if up.contains_key(&c) == false {
                set.insert(c);
            }
        }
        set
    };

    if search(n, row, col, pos + 1, s) {
        return true;
    }
    for &new in x.intersection(&y) {
        s[i][j] = new;
        if search(n, row, col, pos + 1, s) {
            return true;
        }
        s[i][j] = '.';
    }
    false
}

fn main() {
    input! {
        n: usize,
        row: Chars,
        col: Chars
    };

    let mut s = vec![vec!['.'; n]; n];
    let found = search(n, &row, &col, 0, &mut s);
    if found {
        println!("Yes");
        for i in 0..n {
            println!("{}", s[i].iter().collect::<String>());
        }
    } else {
        println!("No");
    }
}
