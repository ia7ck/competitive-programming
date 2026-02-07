use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    };

    a.sort_unstable();

    let a_max = a.iter().max().copied().unwrap();

    let mut digits = Vec::new();
    let mut i = 1;
    let mut carry = 0;
    while i <= a_max || carry > 0 {
        let p = a.partition_point(|&a| a < i);
        let x = (n - p) + carry;
        let d = x % 10;
        digits.push(d);
        i += 1;
        carry = (x - d) / 10;
    }

    while let Some(&last) = digits.last() {
        if last == 0 {
            digits.pop();
        } else {
            break;
        }
    }

    digits.reverse();
    for d in digits {
        print!("{}", d);
    }
    println!();
}
