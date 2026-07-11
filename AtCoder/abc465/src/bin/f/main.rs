use std::ops::Range;

use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        sv: [(Bytes, i64); n],
        q: usize,
        queries: [(Bytes, Bytes); q],
    };

    let mut builder = PrefixSumBuilder::new(&[10, 10, 10, 10, 10, 10]);
    for (s, v) in sv {
        builder.add(&digits(s), v);
    }
    let prefix_sum = builder.build();

    for (x, y) in queries {
        let x = digits(x);
        let y = digits(y);
        if x.iter().zip(&y).any(|(x, y)| x > y) {
            println!("0");
            continue;
        }

        let ranges = [
            x[0]..y[0] + 1,
            x[1]..y[1] + 1,
            x[2]..y[2] + 1,
            x[3]..y[3] + 1,
            x[4]..y[4] + 1,
            x[5]..y[5] + 1,
        ];
        println!("{}", prefix_sum.sum(&ranges));
    }
}

fn digits(s: Vec<u8>) -> Vec<usize> {
    s.into_iter().map(|b| usize::from(b - b'0')).collect()
}

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
        assert!(!shape.is_empty());
        assert!(shape.iter().all(|&length| length > 0));

        let (strides, len) = strides_and_len(shape);
        Self {
            shape: shape.to_vec(),
            strides,
            values: vec![T::zero(); len],
        }
    }

    pub fn add(&mut self, index: &[usize], value: T) {
        let flat_index = flat_index(&self.shape, &self.strides, index);
        self.values[flat_index].add_assign(&value);
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
        assert_eq!(ranges.len(), self.shape.len());
        for (range, &axis_length) in ranges.iter().zip(&self.shape) {
            assert!(range.start <= range.end && range.end <= axis_length);
        }
        if ranges.iter().any(|range| range.start == range.end) {
            return T::zero();
        }

        let corner_count = 1usize << self.shape.len();
        let mut corners = Vec::with_capacity(corner_count);

        for mask in 0..corner_count {
            let mut flat_index = 0;
            let mut is_zero = false;
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

fn strides_and_len(shape: &[usize]) -> (Vec<usize>, usize) {
    let mut strides = vec![0; shape.len()];
    let mut len = 1usize;
    for axis in (0..shape.len()).rev() {
        strides[axis] = len;
        len = len.checked_mul(shape[axis]).unwrap();
    }
    (strides, len)
}

fn flat_index(shape: &[usize], strides: &[usize], index: &[usize]) -> usize {
    assert_eq!(index.len(), shape.len());
    index
        .iter()
        .zip(shape)
        .zip(strides)
        .map(|((&coordinate, &axis_length), &stride)| {
            assert!(coordinate < axis_length);
            coordinate * stride
        })
        .sum()
}

pub trait PrefixSumValue: Clone {
    fn zero() -> Self;
    fn add_assign(&mut self, rhs: &Self);
    fn sub_assign(&mut self, rhs: &Self);
}

impl PrefixSumValue for i64 {
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
