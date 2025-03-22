use std::str::{self, Utf8Error};

use proconio::{input, marker::Bytes};
use rolling_hash::RollingHash;

fn main() -> Result<(), Utf8Error> {
    input! {
        s: Bytes,
    };

    let rh = s.iter().map(|&b| u64::from(b)).collect::<RollingHash>();
    let rh2 = s
        .iter()
        .rev()
        .map(|&b| u64::from(b))
        .collect::<RollingHash>();

    if rh.hash(0..s.len()) == rh2.hash(0..s.len()) {
        let ans = str::from_utf8(&s)?;
        println!("{}", ans);
        return Ok(());
    }

    for i in 1..s.len() {
        // palindrome a[i..] ?
        if rh.hash(i..s.len()) == rh2.hash(0..(s.len() - i)) {
            let (prefix, suffix) = s.split_at(i);
            let rev_prefix = prefix.iter().rev().copied().collect::<Vec<_>>();
            let prefix = str::from_utf8(prefix)?;
            let suffix = str::from_utf8(suffix)?;
            let rev_prefix = str::from_utf8(&rev_prefix)?;
            println!("{}{}{}", prefix, suffix, rev_prefix);
            return Ok(());
        }
    }

    unreachable!();
}
