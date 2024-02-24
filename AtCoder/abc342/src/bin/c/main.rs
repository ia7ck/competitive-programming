use std::mem;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        q: usize,
        cd: [(char, char); q],
    };

    let mut map = vec![vec![]; 26];
    for i in 0..26 {
        map[i] = vec![i];
    }
    for (c, d) in cd {
        let c = c as usize - 'a' as usize;
        let d = d as usize - 'a' as usize;
        let old = mem::take(&mut map[c]);
        assert!(old.len() <= 26);
        for i in old {
            map[d].push(i);
        }
    }

    let map = {
        let mut new = vec![usize::MAX; 26];
        for i in 0..26 {
            for &j in &map[i] {
                new[j] = i;
            }
        }
        for i in 0..26 {
            assert_ne!(new[i], usize::MAX);
        }
        new
    };

    for ch in s {
        let new = map[ch as usize - 'a' as usize];
        print!("{}", (new as u8 + 'a' as u8) as char);
    }
    println!();
}
