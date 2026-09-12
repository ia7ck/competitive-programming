use proconio::{input, marker::Usize1};

use crate::prefix_sum::PrefixSum1DBuilder;

fn main() {
    input! {
        n: usize,
        s: Usize1,
        m: u64,
        a: [u64; n - 1],
    };

    let mut p = PrefixSum1DBuilder::<u64>::new(n - 1);
    for i in 0..(n - 1) {
        p.set(i, a[i]);
    }
    let p = p.build();
    let mut ans = 1;
    for l in 0..=s {
        for r in s..=(n - 1) {
            let l_sum = p.sum(l..s);
            let r_sum = p.sum(s..r);
            let cost = l_sum.min(r_sum) * 2 + l_sum.max(r_sum);
            if cost <= m {
                ans = ans.max(r - l + 1);
            }
        }
    }

    println!("{}", ans);
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod prefix_sum {

    use std::ops::Range;

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

        pub fn set(&mut self, index: &[usize], value: T) {
            let flat_index = flat_index(&self.shape, &self.strides, index);
            self.values[flat_index] = value;
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

        pub fn set(&mut self, index: usize, value: T) {
            self.inner.set(&[index], value);
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
        pub fn sum(&self, range: Range<usize>) -> T {
            self.inner.sum(&[range])
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

        pub fn set(&mut self, row: usize, column: usize, value: T) {
            self.inner.set(&[row, column], value);
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
        pub fn sum(&self, rows: Range<usize>, columns: Range<usize>) -> T {
            self.inner.sum(&[rows, columns])
        }
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
