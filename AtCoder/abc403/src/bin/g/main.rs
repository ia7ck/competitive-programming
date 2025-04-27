use proconio::input;

fn main() {
    input! {
        q: usize,
        queries: [u64; q],
    };

    let bucket_size = 500;
    let mut buckets = Vec::<Bucket>::new();
    buckets.push(Bucket::default());

    let mut ans = Vec::new();
    for y in queries {
        let z = ans.last().unwrap_or(&0);
        let x = (y + z) % 1_000_000_000 + 1; // random ??

        let mut i = buckets.partition_point(|b| match b.sorted_elements.last() {
            None => true,
            Some(&w) => w < x,
        });
        if i == buckets.len() {
            i -= 1;
        }
        let p = buckets[i].sorted_elements.partition_point(|&w| w < x);
        buckets[i].insert(p, x);
        if buckets[i].sorted_elements.len() > bucket_size {
            let large = buckets[i].split_off(bucket_size / 2);
            buckets.insert(i + 1, large);
        }

        // sorted
        debug_assert!(buckets
            .iter()
            .flat_map(|b| &b.sorted_elements)
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| w[0] <= w[1]));

        let mut sum = 0;
        let mut count = 0;
        for b in &buckets {
            if count % 2 == 0 {
                sum += b.sum_odd;
            } else {
                sum += b.sum_even;
            }
            count += b.sorted_elements.len();
        }
        ans.push(sum);
    }

    for ans in ans {
        println!("{}", ans);
    }
}

#[derive(Default, Debug)]
struct Bucket {
    sorted_elements: Vec<u64>, // .len() <= bucket_size
    sum_odd: u64,
    sum_even: u64,
}

impl Bucket {
    fn new(sorted_elements: Vec<u64>) -> Self {
        let mut sum_odd = 0;
        let mut sum_even = 0;
        for (i, &x) in sorted_elements.iter().enumerate() {
            if i % 2 == 0 {
                sum_odd += x;
            } else {
                sum_even += x;
            }
        }
        Self {
            sorted_elements,
            sum_odd,
            sum_even,
        }
    }

    fn insert(&mut self, p: usize, x: u64) {
        self.sorted_elements.insert(p, x);
        *self = Self::new(std::mem::take(&mut self.sorted_elements))
    }

    fn split_off(&mut self, at: usize) -> Self {
        let tail = self.sorted_elements.split_off(at);
        *self = Self::new(std::mem::take(&mut self.sorted_elements));
        Self::new(tail)
    }
}
