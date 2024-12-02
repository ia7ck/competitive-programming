use proconio::input_interactive;

fn main() {
    input_interactive! {
        n: usize,
        _q: usize,
    };

    // a[1..], a[2..], ..., a[n..]
    let (mut min, mut max) = (1, 1);
    for i in (1..=n).step_by(2) {
        let (p, q) = (i, i + 1);
        if query((p, n), (q, n)) {
            // a[p..] < a[q..]
            if query((p, n), (min, n)) {
                min = p;
            }
            if query((max, n), (q, n)) {
                max = q;
            }
        } else {
            // a[p..] > a[q..]
            if query((q, n), (min ,n)) {
                min = q;
            }
            if query((max, n), (p, n)) {
                max = p;
            }
        }
    }

    println!("! {min} {min} {max} {n}");
}

fn query((l1, r1): (usize, usize), (l2, r2): (usize, usize)) -> bool {
    println!("? {l1} {r1} {l2} {r2}");

    input_interactive! {
        less: i8,
    }

    assert_ne!(less, -1);
    less == 1
}
