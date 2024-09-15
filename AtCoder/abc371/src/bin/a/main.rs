use proconio::input;

fn main() {
    input! {
        ab: char,
        ac: char,
        bc: char,
    };

    // a < b < c
    // a < c < b
    // b < a < c
    // b < c < a
    // c < a < b
    // c < b < a
    let ans = match (ab, ac, bc) {
        ('>', '<', '<') | ('<', '>', '>') => 'A',
        ('<', '<', '<') | ('>', '>', '>') => 'B',
        ('<', '<', '>') | ('>', '>', '<') => 'C',
        _ => unreachable!()
    };

    println!("{}", ans);
}
