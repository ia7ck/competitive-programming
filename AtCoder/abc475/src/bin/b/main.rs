use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    let mut changes = Vec::new();
    for a in a {
        let b = (a + 1000 - 1) / 1000 * 1000 - a;
        changes.push((b % 10, (b / 10) % 10, (b / 100) % 10));
    }

    let mut one = 0;
    let mut ten = 0;
    let mut hundred = 0;
    for (one_, ten_, hundred_) in changes {
        one += one_;
        ten += ten_;
        hundred += hundred_;
    }

    println!("{} {} {}", one, ten, hundred);
}
