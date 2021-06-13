use procon_reader::ProconReader;
use std::cmp::Ordering;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let a: i64 = rd.get();
    let b: i64 = rd.get();
    let c: i64 = rd.get();

    use Ordering::*;
    if c % 2 == 0 {
        let a = a * a;
        let b = b * b;
        let ans = match a.cmp(&b) {
            Less => '<',
            Equal => '=',
            Greater => '>',
        };
        println!("{}", ans);
        return;
    }

    let ans = match (a.signum(), b.signum()) {
        (0, 0) => Equal,
        (0, 1) => Less,
        (0, -1) => Greater,
        (1, 0) => Greater,
        (1, 1) => a.cmp(&b),
        (1, -1) => Greater,
        (-1, 0) => Less,
        (-1, 1) => Less,
        (-1, -1) => a.cmp(&b),
        _ => unreachable!(),
    };
    let ans = match ans {
        Less => '<',
        Equal => '=',
        Greater => '>',
    };
    println!("{}", ans);
}
