use proconio::input;

use crate::avl_tree::AvlTree;

fn main() {
    input! {
        n: usize,
        d: i64,
        a: [i64; n],
    };

    let mut ans = 0_usize;
    let mut set = AvlTree::new();
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
mod avl_tree {
    use std::{
        cmp::{self, Ordering},
        fmt,
    };

    #[derive(Clone)]
    struct Node<T> {
        x: T,
        height: i32,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
        size: usize,
    }

    #[derive(Clone)]
    pub struct AvlTree<T> {
        n: usize,
        root: Option<Box<Node<T>>>,
    }

    impl<T> AvlTree<T> {
        pub fn new() -> Self {
            Self { n: 0, root: None }
        }

        pub fn len(&self) -> usize {
            self.n
        }

        pub fn is_empty(&self) -> bool {
            self.n == 0
        }

        fn new_node(x: T) -> Box<Node<T>> {
            Box::new(Node {
                x,
                height: 1,
                left: None,
                right: None,
                size: 1,
            })
        }

        fn node_height(node: &Option<Box<Node<T>>>) -> i32 {
            node.as_ref().map_or(0, |n| n.height)
        }

        fn node_size(node: &Option<Box<Node<T>>>) -> usize {
            node.as_ref().map_or(0, |n| n.size)
        }

        fn balance_factor(node: &Node<T>) -> i32 {
            Self::node_height(&node.left) - Self::node_height(&node.right)
        }

        fn update_height_and_size(node: &mut Node<T>) {
            node.height = 1 + Self::node_height(&node.left).max(Self::node_height(&node.right));
            node.size = 1 + Self::node_size(&node.left) + Self::node_size(&node.right);
        }

        fn rotate_right(mut root: Box<Node<T>>) -> Box<Node<T>> {
            let mut left = root.left.take().unwrap();
            let b = left.right.take();

            root.left = b;
            Self::update_height_and_size(&mut root);

            left.right = Some(root);
            Self::update_height_and_size(&mut left);

            left
        }

        fn rotate_left(mut root: Box<Node<T>>) -> Box<Node<T>> {
            let mut right = root.right.take().unwrap();
            let b = right.left.take();

            root.right = b;
            Self::update_height_and_size(&mut root);

            right.left = Some(root);
            Self::update_height_and_size(&mut right);

            right
        }

        fn rebalance(mut node: Box<Node<T>>) -> Box<Node<T>> {
            Self::update_height_and_size(&mut node);

            let balance = Self::balance_factor(&node);

            if balance > 1 {
                if let Some(left) = node.left.take() {
                    if Self::balance_factor(&left) < 0 {
                        node.left = Some(Self::rotate_left(left));
                    } else {
                        node.left = Some(left);
                    }
                }
                return Self::rotate_right(node);
            }

            if balance < -1 {
                if let Some(right) = node.right.take() {
                    if Self::balance_factor(&right) > 0 {
                        node.right = Some(Self::rotate_right(right));
                    } else {
                        node.right = Some(right);
                    }
                }
                return Self::rotate_left(node);
            }

            node
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

    impl<T> AvlTree<T>
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

        pub fn insert(&mut self, x: T) -> bool {
            let root = self.root.take();
            let mut inserted = false;
            self.root = Self::insert_recursive(root, x, &mut inserted);
            if inserted {
                self.n += 1;
            }
            inserted
        }

        fn insert_recursive(
            root: Option<Box<Node<T>>>,
            x: T,
            inserted: &mut bool,
        ) -> Option<Box<Node<T>>> {
            let mut root = match root {
                Some(root) => root,
                None => {
                    *inserted = true;
                    return Some(Self::new_node(x));
                }
            };

            match x.cmp(&root.x) {
                Ordering::Less => {
                    root.left = Self::insert_recursive(root.left.take(), x, inserted);
                }
                Ordering::Greater => {
                    root.right = Self::insert_recursive(root.right.take(), x, inserted);
                }
                Ordering::Equal => return Some(root),
            }

            if *inserted {
                Some(Self::rebalance(root))
            } else {
                Some(root)
            }
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
                }
                Ordering::Greater => {
                    root.right = Self::remove_recursive(root.right.take(), x, removed);
                }
                Ordering::Equal => {
                    *removed = true;
                    return Self::remove_node(root);
                }
            }

            if *removed {
                Some(Self::rebalance(root))
            } else {
                Some(root)
            }
        }

        fn remove_node(mut node: Box<Node<T>>) -> Option<Box<Node<T>>> {
            match (node.left.take(), node.right.take()) {
                (None, None) => None,
                (None, Some(right)) => Some(right),
                (Some(left), None) => Some(left),
                (Some(left), Some(right)) => {
                    node.left = Some(left);
                    let (successor_value, new_right) = Self::extract_min(right);
                    node.x = successor_value;
                    node.right = new_right;
                    Some(Self::rebalance(node))
                }
            }
        }

        fn extract_min(mut node: Box<Node<T>>) -> (T, Option<Box<Node<T>>>) {
            match node.left.take() {
                None => (node.x, node.right.take()),
                Some(left) => {
                    let (min_value, new_left) = Self::extract_min(left);
                    node.left = new_left;
                    (min_value, Some(Self::rebalance(node)))
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

    impl<T> Default for AvlTree<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> fmt::Debug for AvlTree<T>
    where
        T: fmt::Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.iter()).finish()
        }
    }

    pub struct Iter<'a, T> {
        stack: Vec<&'a Node<T>>,
    }

    impl<'a, T> Iter<'a, T> {
        fn new(root: &'a Option<Box<Node<T>>>) -> Self {
            let mut iter = Self { stack: Vec::new() };
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

    impl<T> AvlTree<T> {
        pub fn iter(&self) -> Iter<'_, T> {
            Iter::new(&self.root)
        }
    }
}
