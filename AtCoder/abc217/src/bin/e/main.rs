use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let q: usize = rd.get();
    use std::collections::{BTreeMap, VecDeque};
    let mut freq = BTreeMap::new();
    let mut tail = VecDeque::new();
    for _ in 0..q {
        let c: u8 = rd.get();
        match c {
            1 => {
                let x: u32 = rd.get();
                tail.push_back(x);
            }
            2 => {
                if let Some((&key, val)) = freq.iter_mut().next() {
                    println!("{}", key);
                    *val -= 1;
                    if *val == 0 {
                        freq.remove(&key);
                    }
                } else {
                    let x = tail.pop_front().unwrap();
                    println!("{}", x);
                }
            }
            3 => {
                for x in tail.drain(..) {
                    *freq.entry(x).or_insert(0) += 1;
                }
            }
            _ => unreachable!(),
        }
    }
}
