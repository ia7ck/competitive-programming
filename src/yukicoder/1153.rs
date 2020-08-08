// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn dfs(g: &Vec<Vec<usize>>, f: &mut Vec<i32>, i: usize, p: usize) {
    let mut seen = std::collections::HashSet::new();
    for &j in &g[i] {
        if j != p {
            dfs(g, f, j, i);
            seen.insert(f[j]);
        }
    }
    let mut x = 0;
    while seen.contains(&x) {
        x += 1;
    }
    f[i] = x;
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        edges: [(usize, usize); n - 1],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }
    let mut f = vec![-1; n];
    dfs(&g, &mut f, 0, std::usize::MAX);
    let mut grundy = vec![-1; n];
    // grundy[0] = f[0];
    let mut h = vec![(std::usize::MAX, -1); n];
    let mut q = std::collections::VecDeque::new();
    q.push_back((0, std::usize::MAX, -1));
    while let Some((i, p, acc)) = q.pop_front() {
        let mut freq = std::collections::HashMap::new();
        for &j in &g[i] {
            if j != p {
                let c = freq.entry(f[j]).or_insert(0);
                *c += 1;
            }
        }
        {
            // grundy[i]
            let c = freq.entry(acc).or_insert(0);
            *c += 1;
            let mut x = 0;
            while freq.contains_key(&x) {
                x += 1;
            }
            grundy[i] = x;
        }
        for &j in &g[i] {
            if j != p {
                if f[j] < grundy[i] && freq.get(&f[j]) == Some(&1) {
                    h[j] = (i, f[j]);
                    q.push_back((j, i, f[j]));
                } else {
                    h[j] = (i, grundy[i]);
                    q.push_back((j, i, grundy[i]));
                }
            }
        }
    }
    let a = a.iter().map(|i| i - 1).collect::<Vec<_>>();
    let sg = a.iter().fold(0, |acc, &i| acc ^ grundy[i]);
    if sg == 0 {
        println!("-1 -1");
        return;
    }
    let mut seen = std::collections::HashSet::new();
    for (idx, &i) in a.iter().enumerate() {
        if !seen.insert(i) {
            continue;
        }
        let (p, pg) = h[i];
        if pg >= 0 && (sg ^ grundy[i] ^ pg) == 0 {
            println!("{} {}", idx + 1, p + 1);
            return;
        }
        for &j in &g[i] {
            if j != p {
                if (sg ^ grundy[i] ^ f[j]) == 0 {
                    println!("{} {}", idx + 1, j + 1);
                    return;
                }
            }
        }
    }
    unreachable!();
}
