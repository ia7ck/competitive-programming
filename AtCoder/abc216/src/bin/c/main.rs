use join::Join;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let mut n: u64 = rd.get();

    let mut ans = Vec::new();
    while n > 0 {
        if n % 2 == 0 {
            ans.push('B');
            n /= 2;
        } else {
            ans.push('A');
            n -= 1;
        }
    }
    ans.reverse();
    println!("{}", ans.iter().join(""));
}

// 5
// 1 -> 2 -> 4 -> 5

// 14
// 1 -> 2 -> 3 -> 6 -> 7 -> 14
