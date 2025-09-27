use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    };

    let mut b = (1..=n).collect::<Vec<_>>();
    let mut p = vec![0; n];
    for i in 0..n {
        if a[i] != -1 {
            if let Some(j) = b.iter().position(|&x| x == a[i] as usize) {
                let x = b.swap_remove(j);
                p[i] = x;
            } else {
                println!("No");
                return;
            }
        }
    }
    for i in 0..n {
        if a[i] == -1 {
            let x = b.pop().unwrap();
            p[i] = x;
        }
    }

    println!("Yes");
    println!(
        "{}",
        p.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
