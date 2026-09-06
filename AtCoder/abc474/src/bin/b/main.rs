use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    for (i, chunk) in p.chunks(10).enumerate() {
        // [i*10+1, (i+1)*10]
        if chunk.iter().all(|&p| i * 10 + 1 <= p && p <= (i + 1) * 10) {
            // ok
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
