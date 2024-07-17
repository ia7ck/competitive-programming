use proconio::{
    input,
    marker::{Bytes, Usize1},
};

fn main() {
    input! {
        n: usize,
        mut s: Bytes,
        q: usize,
    };

    let mut total = vec![0; 26];
    for &c in &s {
        total[usize::from(c - b'a')] += 1;
    }

    let mut f = vec![FenwickTree::new(n, 0_isize); 26];
    for i in 0..n {
        f[usize::from(s[i] - b'a')].add(i, 1);
    }
    for _ in 0..q {
        input! {
            op: u8,
        };
        if op == 1 {
            input! {
                x: Usize1,
                c: char,
            };
            total[usize::from(s[x] - b'a')] -= 1;
            f[usize::from(s[x] - b'a')].add(x, -1);
            s[x] = c as u8;
            total[usize::from(s[x] - b'a')] += 1;
            f[usize::from(s[x] - b'a')].add(x, 1);
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            };
            let mut count = vec![0; 26];
            for i in 0..26 {
                count[i] = f[i].sum(l..(r + 1)) as usize;
            }
            let first = (0..26).find(|&i| count[i] >= 1).unwrap();
            let last = (0..26).rfind(|&i| count[i] >= 1).unwrap();
            let mut ok = first <= last;
            let mut offset = l + count[first];
            for i in (first + 1)..last {
                let c = f[i].sum(offset..(r + 1)) as usize;
                if c == total[i] {
                    offset += c;
                } else {
                    ok = false;
                    break;
                }
            }
            if ok && offset + f[last].sum(offset..(r + 1)) as usize == r + 1 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

#[derive(Clone)]
pub struct FenwickTree<T> {
    n: usize,
    e: T,
    dat: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Copy,
    T: std::ops::AddAssign,
    T: std::ops::SubAssign,
{
    pub fn new(n: usize, e: T) -> Self {
        let n = n.next_power_of_two();
        let mut dat = vec![e; n + 1];
        for i in (2..=n).step_by(2) {
            let j = 1 << (i.trailing_zeros() - 1);
            let mut y = dat[j];
            y += dat[j];
            dat[i] = y; // dat[j] * 2
        }
        Self { n, e, dat }
    }
    // 0-indexed
    // a[k] += x
    pub fn add(&mut self, k: usize, x: T) {
        assert!(k < self.n);
        let mut k = k + 1;
        while k <= self.n {
            self.dat[k] += x;
            k += 1 << k.trailing_zeros();
        }
    }
    // 1-indexed
    // a[1] + a[2] + ... + a[r]
    fn _sum(&self, r: usize) -> T {
        assert!(r <= self.n);
        let mut result = self.e;
        let mut k = r;
        while k >= 1 {
            result += self.dat[k];
            k -= 1 << k.trailing_zeros();
        }
        result
    }
    // 0-indexed
    // a[l] + a[l + 1] + ... + a[r - 1]
    pub fn sum(&self, range: std::ops::Range<usize>) -> T {
        let (l, r) = (range.start, range.end);
        assert!(r <= self.n);
        let mut result = self._sum(r);
        result -= self._sum(l);
        result
    }
}
