use procon_reader::ProconReader;
use std::cmp::Reverse;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let q: usize = rd.get();
    let mut acc = 0;
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    for _ in 0..q {
        let tp: u8 = rd.get();
        match tp {
            1 => {
                let x: i64 = rd.get();
                heap.push(Reverse(x - acc));
            }
            2 => {
                let x: i64 = rd.get();
                acc += x;
            }
            3 => {
                let Reverse(x) = heap.pop().unwrap();
                println!("{}", x + acc);
            }
            _ => unreachable!(),
        }
    }
}
