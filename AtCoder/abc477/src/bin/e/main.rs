use proconio::{input, marker::Usize1};

use crate::{
    dijkstra::{ConstEdge, dijkstra},
    prefix_sum::PrefixSum1DBuilder,
};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        b: [i64; n],
        queries: [(Usize1, Usize1); q],
    };

    let mut p = PrefixSum1DBuilder::new(n * 2);
    for i in 0..n {
        p.set(i, a[i]);
    }
    for i in 0..n {
        p.set(n + i, a[i]);
    }
    let p = p.build();

    let mut edges = Vec::<ConstEdge<i64>>::new();
    for i in 0..n {
        edges.push(ConstEdge::new(i, (i + 1) % n, a[i]));
        edges.push(ConstEdge::new((i + 1) % n, i, a[i]));

        edges.push(ConstEdge::new(i, n, b[i]));
        edges.push(ConstEdge::new(n, i, b[i]));
    }
    let (d, _) = dijkstra(n + 1, &edges, n);
    let d = d.into_iter().map(Option::unwrap).collect::<Vec<_>>();

    for (s, t) in queries {
        if t == n {
            println!("{}", d[s]);
            continue;
        }

        let s_t = p.sum(s..t);
        let t_s = p.sum(t..(s + n));
        let s_n_t = d[s] + d[t];

        println!("{}", s_t.min(t_s).min(s_n_t));
    }
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod prefix_sum {

    use std::ops::{Bound, Range, RangeBounds};

    pub struct PrefixSumBuilder<T> {
        shape: Vec<usize>,
        strides: Vec<usize>,
        values: Vec<T>,
    }

    impl<T> PrefixSumBuilder<T>
    where
        T: PrefixSumValue,
    {
        pub fn new(shape: &[usize]) -> Self {
            assert!(!shape.is_empty(), "shape must have at least one dimension");
            assert!(
                shape.iter().all(|&length| length > 0),
                "each axis length must be positive"
            );

            let (strides, len) = strides_and_len(shape);
            Self {
                shape: shape.to_vec(),
                strides,
                values: vec![T::zero(); len],
            }
        }

        pub fn get(&self, index: &[usize]) -> &T {
            let flat_index = flat_index(&self.shape, &self.strides, index);
            &self.values[flat_index]
        }

        pub fn set(&mut self, index: &[usize], value: T) {
            let flat_index = flat_index(&self.shape, &self.strides, index);
            self.values[flat_index] = value;
        }

        pub fn update(&mut self, index: &[usize], f: impl FnOnce(&T) -> T) {
            let flat_index = flat_index(&self.shape, &self.strides, index);
            self.values[flat_index] = f(&self.values[flat_index]);
        }

        pub fn build(mut self) -> PrefixSum<T> {
            for axis in 0..self.shape.len() {
                let stride = self.strides[axis];
                let axis_length = self.shape[axis];
                let block_length = stride * axis_length;

                for block_start in (0..self.values.len()).step_by(block_length) {
                    for coordinate in 1..axis_length {
                        let start = block_start + coordinate * stride;
                        for offset in 0..stride {
                            let previous = self.values[start - stride + offset].clone();
                            self.values[start + offset].add_assign(&previous);
                        }
                    }
                }
            }

            PrefixSum {
                shape: self.shape,
                strides: self.strides,
                values: self.values,
            }
        }
    }

    pub struct PrefixSum<T> {
        shape: Vec<usize>,
        strides: Vec<usize>,
        values: Vec<T>,
    }

    impl<T> PrefixSum<T>
    where
        T: PrefixSumValue,
    {
        pub fn sum(&self, ranges: &[Range<usize>]) -> T {
            assert_eq!(
                ranges.len(),
                self.shape.len(),
                "range rank must match shape rank"
            );
            for (range, &axis_length) in ranges.iter().zip(&self.shape) {
                assert!(
                    range.start <= range.end && range.end <= axis_length,
                    "range must be within the shape"
                );
            }
            if ranges.iter().any(|range| range.start == range.end) {
                return T::zero();
            }

            assert!(
                self.shape.len() < usize::BITS as usize,
                "rank is too large to enumerate query corners"
            );
            let corner_count = 1usize << self.shape.len();
            let mut corners = Vec::with_capacity(corner_count);

            for mask in 0..corner_count {
                let mut flat_index = 0;
                let mut is_zero = false;
                #[expect(clippy::needless_range_loop)]
                for axis in 0..self.shape.len() {
                    let endpoint = if mask & (1usize << axis) == 0 {
                        ranges[axis].start
                    } else {
                        ranges[axis].end
                    };
                    if endpoint == 0 {
                        is_zero = true;
                        break;
                    }
                    flat_index += (endpoint - 1) * self.strides[axis];
                }
                corners.push(if is_zero {
                    T::zero()
                } else {
                    self.values[flat_index].clone()
                });
            }

            for axis in 0..self.shape.len() {
                let bit = 1usize << axis;
                for mask in 0..corner_count {
                    if mask & bit != 0 {
                        let lower = corners[mask ^ bit].clone();
                        corners[mask].sub_assign(&lower);
                    }
                }
            }

            corners.pop().unwrap()
        }
    }

    pub struct PrefixSum1DBuilder<T> {
        inner: PrefixSumBuilder<T>,
    }

    impl<T> PrefixSum1DBuilder<T>
    where
        T: PrefixSumValue,
    {
        pub fn new(length: usize) -> Self {
            Self {
                inner: PrefixSumBuilder::new(&[length]),
            }
        }

        pub fn get(&self, index: usize) -> &T {
            self.inner.get(&[index])
        }

        pub fn set(&mut self, index: usize, value: T) {
            self.inner.set(&[index], value);
        }

        pub fn update(&mut self, index: usize, f: impl FnOnce(&T) -> T) {
            self.inner.update(&[index], f);
        }

        pub fn build(self) -> PrefixSum1D<T> {
            PrefixSum1D {
                inner: self.inner.build(),
            }
        }
    }

    pub struct PrefixSum1D<T> {
        inner: PrefixSum<T>,
    }

    impl<T> PrefixSum1D<T>
    where
        T: PrefixSumValue,
    {
        pub fn sum(&self, range: impl RangeBounds<usize>) -> T {
            self.inner.sum(&[to_range(range, self.inner.shape[0])])
        }
    }

    pub struct PrefixSum2DBuilder<T> {
        inner: PrefixSumBuilder<T>,
    }

    impl<T> PrefixSum2DBuilder<T>
    where
        T: PrefixSumValue,
    {
        pub fn new(height: usize, width: usize) -> Self {
            Self {
                inner: PrefixSumBuilder::new(&[height, width]),
            }
        }

        pub fn get(&self, row: usize, column: usize) -> &T {
            self.inner.get(&[row, column])
        }

        pub fn set(&mut self, row: usize, column: usize, value: T) {
            self.inner.set(&[row, column], value);
        }

        pub fn update(&mut self, row: usize, column: usize, f: impl FnOnce(&T) -> T) {
            self.inner.update(&[row, column], f);
        }

        pub fn build(self) -> PrefixSum2D<T> {
            PrefixSum2D {
                inner: self.inner.build(),
            }
        }
    }

    pub struct PrefixSum2D<T> {
        inner: PrefixSum<T>,
    }

    impl<T> PrefixSum2D<T>
    where
        T: PrefixSumValue,
    {
        pub fn sum(&self, rows: impl RangeBounds<usize>, columns: impl RangeBounds<usize>) -> T {
            let rows = to_range(rows, self.inner.shape[0]);
            let columns = to_range(columns, self.inner.shape[1]);
            self.inner.sum(&[rows, columns])
        }
    }

    fn to_range(range: impl RangeBounds<usize>, length: usize) -> Range<usize> {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start.checked_add(1).expect("range start overflows usize"),
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end.checked_add(1).expect("range end overflows usize"),
            Bound::Excluded(&end) => end,
            Bound::Unbounded => length,
        };
        start..end
    }

    fn strides_and_len(shape: &[usize]) -> (Vec<usize>, usize) {
        let mut strides = vec![0; shape.len()];
        let mut len = 1usize;
        for axis in (0..shape.len()).rev() {
            strides[axis] = len;
            len = len
                .checked_mul(shape[axis])
                .expect("shape is too large to fit in usize");
        }
        (strides, len)
    }

    fn flat_index(shape: &[usize], strides: &[usize], index: &[usize]) -> usize {
        assert_eq!(index.len(), shape.len(), "index rank must match shape rank");
        index
            .iter()
            .zip(shape)
            .zip(strides)
            .map(|((&coordinate, &axis_length), &stride)| {
                assert!(coordinate < axis_length, "index must be within the shape");
                coordinate * stride
            })
            .sum()
    }

    pub trait PrefixSumValue: Clone {
        fn zero() -> Self;
        fn add_assign(&mut self, rhs: &Self);
        fn sub_assign(&mut self, rhs: &Self);
    }

    macro_rules! impl_prefix_sum_value {
        ($($t:ty),+ $(,)?) => {
            $(
                impl PrefixSumValue for $t {
                    fn zero() -> Self {
                        0
                    }

                    fn add_assign(&mut self, rhs: &Self) {
                        *self += *rhs;
                    }

                    fn sub_assign(&mut self, rhs: &Self) {
                        *self -= *rhs;
                    }
                }
            )+
        };
    }

    impl_prefix_sum_value!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
    );
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod dijkstra {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::fmt::Debug;
    use std::ops::Add;

    pub trait Edge<T> {
        fn from(&self) -> usize;
        fn to(&self) -> usize;
        fn dist(&self, d: &T) -> T;
    }

    pub struct ConstEdge<T> {
        from: usize,
        to: usize,
        cost: T,
    }

    impl<T> ConstEdge<T> {
        pub fn new(from: usize, to: usize, cost: T) -> Self {
            Self { from, to, cost }
        }
    }

    impl<T> Edge<T> for ConstEdge<T>
    where
        T: Copy + Add<Output = T>,
    {
        fn from(&self) -> usize {
            self.from
        }
        fn to(&self) -> usize {
            self.to
        }
        fn dist(&self, d: &T) -> T {
            *d + self.cost
        }
    }

    pub fn dijkstra<E, T>(n: usize, edges: &[E], s: usize) -> (Vec<Option<T>>, Vec<Option<usize>>)
    where
        E: Edge<T>,
        T: Clone + Default + Ord + Debug,
    {
        let mut adj = vec![vec![]; n];
        for e in edges {
            adj[e.from()].push(e);
        }
        let mut dist = vec![Option::<T>::None; n];
        let mut heap = BinaryHeap::new();
        let mut prev = vec![None; n];
        dist[s] = Some(T::default());
        heap.push((Reverse(T::default()), s));
        while let Some((Reverse(d), v)) = heap.pop() {
            #[allow(clippy::comparison_chain)]
            match &dist[v] {
                Some(dv) => {
                    if dv < &d {
                        continue;
                    } else if dv > &d {
                        unreachable!();
                    } else {
                        assert_eq!(dv, &d);
                    }
                }
                None => unreachable!(),
            }
            for e in &adj[v] {
                let next_d = e.dist(&d);
                let to = e.to();
                match &dist[to] {
                    Some(dt) if dt <= &next_d => {
                        continue;
                    }
                    _ => {
                        dist[to] = Some(next_d.clone());
                        prev[to] = Some(v);
                        heap.push((Reverse(next_d), to));
                    }
                }
            }
        }
        (dist, prev)
    }
}
