use proconio::input;

use crate::treap::Treap;

fn main() {
    input! {
        n: usize,
        d: i64,
        a: [i64; n],
    };

    let mut ans = 0_usize;
    let mut set = Treap::default();
    set.insert(a[0]);
    let mut i = 0;
    for j in 1..n {
        loop {
            let prev = set.le(&a[j]).copied().unwrap_or(i64::MIN);
            let next = set.ge(&a[j]).copied().unwrap_or(i64::MAX);
            if prev + d <= a[j] && a[j] + d <= next {
                break;
            } else {
                set.remove(&a[i]);
                i += 1;
            }
        }
        set.insert(a[j]);
        ans += j - i;
    }

    // (j, j)
    ans += n;

    println!("{}", ans);
}

// Bundled
#[rustfmt::skip]
#[allow(unused)]
mod treap {
    use std::{
        cmp::{self, Ordering},
        fmt,
        marker::PhantomData,
    };

    use rand::{RngCore, SeedableRng, rngs::StdRng};

    struct Node<T> {
        x: T,
        priority: u64,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
        size: usize,
    }

    pub struct Treap<T, R> {
        n: usize,
        root: Option<Box<Node<T>>>,
        rng: R,
    }

    impl<T, R> Treap<T, R> {
        pub fn new(rng: R) -> Self {
            Self {
                n: 0,
                root: None,
                rng,
            }
        }

        pub fn len(&self) -> usize {
            self.n
        }

        pub fn is_empty(&self) -> bool {
            self.n == 0
        }

        fn new_node(x: T, priority: u64) -> Box<Node<T>> {
            Box::new(Node {
                x,
                priority,
                left: None,
                right: None,
                size: 1,
            })
        }

        fn rotate_right(mut root: Box<Node<T>>) -> Box<Node<T>> {
            let mut left = root.left.take().unwrap();
            let b = left.right.take();
            root.left = b;

            root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);
            left.size = 1 + Self::node_size(&left.left) + root.size;

            left.right = Some(root);
            left
        }

        fn rotate_left(mut root: Box<Node<T>>) -> Box<Node<T>> {
            let mut right = root.right.take().unwrap();
            let b = right.left.take();
            root.right = b;

            root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);
            right.size = 1 + root.size + Self::node_size(&right.right);

            right.left = Some(root);
            right
        }

        fn node_size(node: &Option<Box<Node<T>>>) -> usize {
            node.as_ref().map_or(0, |n| n.size)
        }

        pub fn into_sorted_vec(mut self) -> Vec<T> {
            fn collect<T>(node: Option<Box<Node<T>>>, acc: &mut Vec<T>) {
                if let Some(node) = node {
                    collect(node.left, acc);
                    acc.push(node.x);
                    collect(node.right, acc);
                }
            }

            let mut result = Vec::with_capacity(self.n);
            collect(self.root.take(), &mut result);
            self.n = 0;
            result
        }
    }

    impl<T, R> Treap<T, R>
    where
        R: RngCore,
    {
        fn gen_priority(&mut self) -> u64 {
            self.rng.next_u64()
        }
    }

    impl<T, R> Treap<T, R>
    where
        T: cmp::Ord,
    {
        fn find_last(&self, x: &T) -> Option<&Node<T>> {
            let mut current = &self.root;
            let mut last = Option::<&Node<T>>::None;

            while let Some(node) = current {
                last = Some(node);
                match x.cmp(&node.x) {
                    Ordering::Less => current = &node.left,
                    Ordering::Greater => current = &node.right,
                    Ordering::Equal => return Some(node),
                }
            }

            last
        }

        pub fn contains(&self, x: &T) -> bool {
            self.find_last(x).is_some_and(|node| x.eq(&node.x))
        }

        pub fn remove(&mut self, x: &T) -> bool {
            let root = self.root.take();
            let mut removed = false;
            self.root = Self::remove_recursive(root, x, &mut removed);
            if removed {
                self.n -= 1;
            }
            removed
        }

        fn remove_recursive(
            root: Option<Box<Node<T>>>,
            x: &T,
            removed: &mut bool,
        ) -> Option<Box<Node<T>>> {
            let mut root = root?;

            match x.cmp(&root.x) {
                Ordering::Less => {
                    root.left = Self::remove_recursive(root.left.take(), x, removed);
                    if *removed {
                        root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);
                    }
                    Some(root)
                }
                Ordering::Greater => {
                    root.right = Self::remove_recursive(root.right.take(), x, removed);
                    if *removed {
                        root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);
                    }
                    Some(root)
                }
                Ordering::Equal => {
                    *removed = true;
                    Self::remove_node(root)
                }
            }
        }

        fn remove_node(mut node: Box<Node<T>>) -> Option<Box<Node<T>>> {
            match (&node.left, &node.right) {
                (None, None) => None,
                (None, Some(_)) => node.right.take(),
                (Some(_), None) => node.left.take(),
                (Some(left), Some(right)) => {
                    if left.priority > right.priority {
                        let mut new_root = Self::rotate_right(node);
                        new_root.right = Self::remove_node(new_root.right.take().unwrap());
                        new_root.size =
                            1 + Self::node_size(&new_root.left) + Self::node_size(&new_root.right);
                        Some(new_root)
                    } else {
                        let mut new_root = Self::rotate_left(node);
                        new_root.left = Self::remove_node(new_root.left.take().unwrap());
                        new_root.size =
                            1 + Self::node_size(&new_root.left) + Self::node_size(&new_root.right);
                        Some(new_root)
                    }
                }
            }
        }

        pub fn le(&self, x: &T) -> Option<&T> {
            let mut current = &self.root;
            let mut result = None;

            while let Some(node) = current {
                match x.cmp(&node.x) {
                    Ordering::Less => current = &node.left,
                    Ordering::Greater => {
                        result = Some(&node.x);
                        current = &node.right;
                    }
                    Ordering::Equal => return Some(&node.x),
                }
            }

            result
        }

        pub fn ge(&self, x: &T) -> Option<&T> {
            let mut current = &self.root;
            let mut result = None;

            while let Some(node) = current {
                match x.cmp(&node.x) {
                    Ordering::Less => {
                        result = Some(&node.x);
                        current = &node.left;
                    }
                    Ordering::Greater => current = &node.right,
                    Ordering::Equal => return Some(&node.x),
                }
            }

            result
        }

        pub fn nth(&self, n: usize) -> Option<&T> {
            if n >= self.len() {
                return None;
            }

            let mut current = &self.root;
            let mut n = n;

            while let Some(node) = current {
                let left_size = Self::node_size(&node.left);
                match n.cmp(&left_size) {
                    Ordering::Less => current = &node.left,
                    Ordering::Equal => return Some(&node.x),
                    Ordering::Greater => {
                        n -= 1 + left_size;
                        current = &node.right;
                    }
                }
            }

            unreachable!()
        }

        pub fn position(&self, x: &T) -> Result<usize, usize> {
            let mut current = &self.root;
            let mut count = 0;
            let mut hit = false;

            while let Some(node) = current {
                match x.cmp(&node.x) {
                    Ordering::Less => current = &node.left,
                    Ordering::Equal => {
                        hit = true;
                        current = &node.left;
                    }
                    Ordering::Greater => {
                        count += 1 + Self::node_size(&node.left);
                        current = &node.right;
                    }
                }
            }

            if hit { Ok(count) } else { Err(count) }
        }
    }

    impl<T, R> Treap<T, R>
    where
        T: cmp::Ord,
        R: RngCore,
    {
        pub fn insert(&mut self, x: T) -> bool {
            let root = self.root.take();
            let mut inserted = false;
            self.root = self.insert_recursive(root, x, &mut inserted);
            if inserted {
                self.n += 1;
            }
            inserted
        }

        fn insert_recursive(
            &mut self,
            root: Option<Box<Node<T>>>,
            x: T,
            inserted: &mut bool,
        ) -> Option<Box<Node<T>>> {
            let mut root = match root {
                Some(root) => root,
                None => {
                    *inserted = true;
                    return Some(Self::new_node(x, self.gen_priority()));
                }
            };

            match x.cmp(&root.x) {
                Ordering::Less => {
                    root.left = self.insert_recursive(root.left.take(), x, inserted);
                    if *inserted {
                        root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);

                        if let Some(left) = &root.left
                            && left.priority > root.priority
                        {
                            return Some(Self::rotate_right(root));
                        }
                    }
                    Some(root)
                }
                Ordering::Greater => {
                    root.right = self.insert_recursive(root.right.take(), x, inserted);
                    if *inserted {
                        root.size = 1 + Self::node_size(&root.left) + Self::node_size(&root.right);

                        if let Some(right) = &root.right
                            && right.priority > root.priority
                        {
                            return Some(Self::rotate_left(root));
                        }
                    }
                    Some(root)
                }
                Ordering::Equal => Some(root),
            }
        }
    }

    impl<T> Default for Treap<T, StdRng> {
        fn default() -> Self {
            Self::new(StdRng::seed_from_u64(12233344455555))
        }
    }

    impl<T, R> fmt::Debug for Treap<T, R>
    where
        T: fmt::Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.iter()).finish()
        }
    }

    pub struct Iter<'a, T> {
        stack: Vec<&'a Node<T>>,
        _phantom: PhantomData<&'a T>,
    }

    impl<'a, T> Iter<'a, T> {
        fn new(root: &'a Option<Box<Node<T>>>) -> Self {
            let mut iter = Self {
                stack: Vec::new(),
                _phantom: PhantomData,
            };
            iter.push_left_path(root);
            iter
        }

        fn push_left_path(&mut self, mut node: &'a Option<Box<Node<T>>>) {
            while let Some(n) = node {
                self.stack.push(n);
                node = &n.left;
            }
        }
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            let node = self.stack.pop()?;
            let result = &node.x;
            self.push_left_path(&node.right);
            Some(result)
        }
    }

    impl<T, R> Treap<T, R> {
        pub fn iter(&self) -> Iter<'_, T> {
            Iter::new(&self.root)
        }
    }
}
