use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u16,
        r: [u16; n],
    };

    let mut a = vec![vec![]];
    for i in 0..n {
        let mut new_a = Vec::new();
        for b in a {
            for x in 1..=r[i] {
                let mut b = b.clone();
                b.push(x);
                new_a.push(b);
            }
        }
        a = new_a;
    }

    a.sort();
    for a in a {
        if a.iter().sum::<u16>() % k == 0 {
            println!(
                "{}",
                a.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    }
}
