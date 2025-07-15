use std::{
    alloc,
    cmp::{self, Ordering},
    collections::BTreeSet,
    fmt, ptr,
};

use fenwick_tree::FenwickTree;
use proconio::input;
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use zarts::SortedSeq;

fn main() {
    input! {
        q: usize,
        k: u64,
        queries: [(u8, u64); q],
    };

    let z = queries.iter().map(|&(_, x)| x).collect::<SortedSeq<_>>();
    let mut ft = FenwickTree::new(z.size(), 0);
    let mut pts = BTreeSet::new();
    let mut start = Treap::new();
    for (op, x) in queries {
        if op == 1 {
            let ix = z.ord(&x);
            if ft.sum(ix..=ix) == 0 {
                // add
                ft.add(ix, 1);
                pts.insert(x);

                let l = start.position(&x).unwrap_err();
                if l == 0 {
                    start.add(x);
                    if let Some(&rx) = start.nth(1) {
                        if rx - x <= k {
                            start.remove(&rx);
                        }
                    }
                    continue;
                }

                let lx = pts.range(..x).last().copied().unwrap();
                match start.nth(l) {
                    Some(&rx) => {
                        if x - lx <= k && rx - x <= k {
                            start.remove(&rx);
                        } else if x - lx <= k {
                            //
                        } else if rx - x <= k {
                            start.remove(&rx);
                            start.add(x);
                        } else {
                            start.add(x);
                        }
                    }
                    None => {
                        if x - lx > k {
                            start.add(x);
                        }
                    }
                }
            } else {
                // delete
                ft.add(ix, -1);
                pts.remove(&x);

                match start.position(&x) {
                    Ok(_l) => {
                        start.remove(&x);
                        if let Some(&rx) = pts.range(x..).next() {
                            start.add(rx);
                        }
                    }
                    Err(_l) => match (pts.range(..x).last(), pts.range(x..).next()) {
                        (None, None) => {
                            start.remove(&x);
                        }
                        (None, Some(&rx)) => {
                            start.add(rx);
                        }
                        (Some(&_lx), None) => {
                            start.remove(&x);
                        }
                        (Some(&lx), Some(&rx)) => {
                            if rx - lx > k {
                                start.add(rx);
                            }
                        }
                    },
                }
            }
        } else {
            let ans = match start.position(&x) {
                Ok(i) => {
                    let r = start.nth(i + 1).copied();
                    ft.sum(z.ord(&x)..r.map_or(z.size(), |r| z.ord(&r)))
                }
                Err(i) => {
                    assert!(i >= 1);
                    let l = start.nth(i - 1).copied().unwrap();
                    let r = start.nth(i).copied();
                    ft.sum(z.ord(&l)..r.map_or(z.size(), |r| z.ord(&r)))
                }
            };
            println!("{}", ans);
        }
    }
}

struct Node<T> {
    x: T,
    priority: u64,
    parent: *mut Node<T>,
    left: *mut Node<T>,
    right: *mut Node<T>,
    size: usize,
}

pub struct Treap<T> {
    n: usize,
    root: *mut Node<T>,
    rng: SmallRng,
}

impl<T> Treap<T> {
    pub fn new() -> Self {
        Self {
            n: 0,
            root: ptr::null_mut(),
            rng: SmallRng::seed_from_u64(1223334444),
        }
    }

    fn gen_priority(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn rotate_right(&mut self, u: *mut Node<T>) {
        //         u                      w
        //         |                      |
        //     +---+---+              +---+---+
        //     |       |              |       |
        //     w       c      ->      a       u
        //     |                              |
        // +---+---+                      +---+---+
        // |       |                      |       |
        // a       b                      b       c
        let w = unsafe { &*u }.left;
        debug_assert_ne!(w, ptr::null_mut());
        let p = unsafe { (*u).parent };
        if p.is_null() {
            debug_assert_eq!(self.root, u);
            self.root = w;
            unsafe { (*w).parent = ptr::null_mut() };
        } else {
            unsafe { (*w).parent = p };
            if unsafe { (*p).left } == u {
                unsafe { (*p).left = w };
            } else {
                debug_assert_eq!(unsafe { (*p).right }, u);
                unsafe { (*p).right = w };
            }
        }
        unsafe { (*u).parent = w };
        let b = unsafe { (*w).right };
        if b.is_null() {
            unsafe { (*u).left = ptr::null_mut() };
        } else {
            unsafe { (*b).parent = u };
            unsafe { (*u).left = b };
        }
        unsafe { (*w).right = u };
        unsafe { (*u).size = 1 + Self::node_size(b) + Self::node_size((*u).right) };
        unsafe { (*w).size = 1 + Self::node_size((*w).left) + (*u).size };
    }

    fn rotate_left(&mut self, u: *mut Node<T>) {
        //      u                         w
        //      |                         |
        //  +---+---+                 +---+---+
        //  |       |                 |       |
        //  a       w        ->       u       c
        //          |                 |
        //      +---+---+         +---+---+
        //      |       |         |       |
        //      b       c         a       b
        let w = unsafe { &*u }.right;
        debug_assert_ne!(w, ptr::null_mut());
        let p = unsafe { (*u).parent };
        if p.is_null() {
            debug_assert_eq!(self.root, u);
            self.root = w;
            unsafe { (*w).parent = ptr::null_mut() };
        } else {
            unsafe { (*w).parent = p };
            if unsafe { (*p).left } == u {
                unsafe { (*p).left = w };
            } else {
                debug_assert_eq!(unsafe { (*p).right }, u);
                unsafe { (*p).right = w };
            }
        }
        unsafe { (*u).parent = w };
        let b = unsafe { (*w).left };
        if b.is_null() {
            unsafe { (*u).right = ptr::null_mut() };
        } else {
            unsafe { (*b).parent = u };
            unsafe { (*u).right = b };
        }
        unsafe { (*w).left = u };
        unsafe { (*u).size = 1 + Self::node_size((*u).left) + Self::node_size(b) };
        unsafe { (*w).size = 1 + (*u).size + Self::node_size((*w).right) };
    }

    fn node_size(u: *mut Node<T>) -> usize {
        if u.is_null() {
            0
        } else {
            unsafe { &*u }.size
        }
    }
}

trait SSet<T> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> bool;
    fn le(&self, x: &T) -> Option<&T>;
    fn ge(&self, x: &T) -> Option<&T>;
    fn nth(&self, n: usize) -> Option<&T>;
    fn position(&self, x: &T) -> Result<usize, usize>;
}

impl<T> Treap<T>
where
    T: cmp::Ord,
{
    fn find_last(&self, x: &T) -> *mut Node<T> {
        let mut w = self.root;
        let mut prev = ptr::null_mut();
        while !w.is_null() {
            prev = w;
            match x.cmp(&unsafe { &*w }.x) {
                Ordering::Less => {
                    w = unsafe { &*w }.left;
                }
                Ordering::Greater => {
                    w = unsafe { &*w }.right;
                }
                Ordering::Equal => {
                    return w;
                }
            }
        }
        prev
    }

    fn add_child(&mut self, p: *mut Node<T>, x: T) -> *mut Node<T> {
        let u = if p.is_null() {
            debug_assert_eq!(self.root, ptr::null_mut());
            self.root = Box::into_raw(Box::new(Node {
                x,
                priority: self.gen_priority(),
                parent: ptr::null_mut(),
                left: ptr::null_mut(),
                right: ptr::null_mut(),
                size: 1,
            }));
            self.root
        } else {
            let y = &unsafe { &*p }.x;
            let ord = x.cmp(y);

            let u = Box::into_raw(Box::new(Node {
                x,
                priority: self.gen_priority(),
                parent: p,
                left: ptr::null_mut(),
                right: ptr::null_mut(),
                size: 1,
            }));

            match ord {
                Ordering::Less => {
                    debug_assert_eq!(unsafe { &*p }.left, ptr::null_mut());
                    unsafe { (*p).left = u };
                    u
                }
                Ordering::Greater => {
                    debug_assert_eq!(unsafe { &*p }.right, ptr::null_mut());
                    unsafe { (*p).right = u };
                    u
                }
                Ordering::Equal => {
                    unreachable!();
                }
            }
        };

        self.n += 1;
        u
    }
}

impl<T> SSet<T> for Treap<T>
where
    T: cmp::Ord,
{
    fn size(&self) -> usize {
        self.n
    }

    fn add(&mut self, x: T) -> bool {
        let p = self.find_last(&x);
        if !p.is_null() && unsafe { &*p }.x.eq(&x) {
            return false;
        }

        let u = self.add_child(p, x);
        // bubble up
        loop {
            let p = unsafe { &*u }.parent;
            if p.is_null() {
                break;
            }
            if unsafe { &*p }.priority < unsafe { &*u }.priority {
                break;
            }
            if unsafe { &*p }.right == u {
                self.rotate_left(p);
            } else if unsafe { &*p }.left == u {
                self.rotate_right(p);
            } else {
                unreachable!();
            }
        }
        if unsafe { &*u }.parent.is_null() {
            self.root = u;
        }

        // update size
        let mut u = u;
        while u != self.root {
            unsafe { (*u).size = 1 + Self::node_size((*u).left) + Self::node_size((*u).right) };
            u = unsafe { &*u }.parent;
        }
        unsafe { (*u).size = 1 + Self::node_size((*u).left) + Self::node_size((*u).right) };

        true
    }

    fn remove(&mut self, x: &T) -> bool {
        let u = self.find_last(x);
        if u.is_null() {
            // 空の状態から削除しようとしたとき
            return false;
        }
        if !unsafe { &*u }.x.eq(x) {
            return false;
        }

        // trickle down
        loop {
            let left = unsafe { &*u }.left;
            let right = unsafe { &*u }.right;
            if left.is_null() && right.is_null() {
                if self.root == u {
                    self.root = ptr::null_mut();
                } else {
                    let p = unsafe { &*u }.parent;
                    debug_assert_ne!(p, ptr::null_mut());
                    if unsafe { &*p }.left == u {
                        unsafe { (*p).left = ptr::null_mut() };
                    } else if unsafe { &*p }.right == u {
                        unsafe { (*p).right = ptr::null_mut() };
                    } else {
                        unreachable!();
                    }

                    // update size
                    let mut v = p;
                    while v != self.root {
                        unsafe {
                            (*v).size = 1 + Self::node_size((*v).left) + Self::node_size((*v).right)
                        };
                        v = unsafe { &*v }.parent;
                    }
                    unsafe {
                        (*v).size = 1 + Self::node_size((*v).left) + Self::node_size((*v).right)
                    };
                }
                unsafe { ptr::drop_in_place(u) };
                unsafe { alloc::dealloc(u as *mut u8, alloc::Layout::new::<Node<T>>()) };
                break;
            }
            if left.is_null() {
                self.rotate_left(u);
            } else if right.is_null() {
                self.rotate_right(u);
            } else if unsafe { &*left }.priority < unsafe { &*right }.priority {
                self.rotate_right(u);
            } else {
                self.rotate_left(u);
            }
        }
        self.n -= 1;
        true
    }

    fn le(&self, x: &T) -> Option<&T> {
        let mut w = self.root;
        let mut z = None; // z.x <= x
        while !w.is_null() {
            let y = &unsafe { &*w }.x;
            match x.cmp(y) {
                Ordering::Less => {
                    w = unsafe { &*w }.left;
                }
                Ordering::Greater => {
                    z = Some(w);
                    w = unsafe { &*w }.right;
                }
                Ordering::Equal => {
                    return Some(y);
                }
            }
        }

        z.map(|z| &unsafe { &*z }.x)
    }

    fn ge(&self, x: &T) -> Option<&T> {
        let mut w = self.root;
        let mut z = None; // z.x >= x
        while !w.is_null() {
            let y = &unsafe { &*w }.x;
            match x.cmp(y) {
                Ordering::Less => {
                    z = Some(w);
                    w = unsafe { &*w }.left;
                }
                Ordering::Greater => {
                    w = unsafe { &*w }.right;
                }
                Ordering::Equal => {
                    return Some(y);
                }
            }
        }

        z.map(|z| &unsafe { &*z }.x)
    }

    // 0-indexed
    fn nth(&self, n: usize) -> Option<&T> {
        if n >= self.size() {
            return None;
        }
        let mut w = self.root;
        let mut n = n;
        while !w.is_null() {
            let left_size = Self::node_size(unsafe { &*w }.left);
            match n.cmp(&left_size) {
                Ordering::Less => {
                    w = unsafe { &*w }.left;
                }
                Ordering::Equal => {
                    return Some(&unsafe { &*w }.x);
                }
                Ordering::Greater => {
                    n -= 1 + left_size;
                    w = unsafe { &*w }.right;
                }
            }
        }
        unreachable!()
    }

    // x未満の個数
    // xが含まれている場合 Ok
    // xが含まれていない場合 Err
    fn position(&self, x: &T) -> Result<usize, usize> {
        let mut w = self.root;
        let mut count = 0;
        let mut hit = false;
        while !w.is_null() {
            let y = &unsafe { &*w }.x;
            if x == y {
                hit = true;
                w = unsafe { &*w }.left;
            } else if x < y {
                w = unsafe { &*w }.left;
            } else {
                count += 1 + Self::node_size(unsafe { &*w }.left);
                w = unsafe { &*w }.right;
            }
        }

        if hit {
            Ok(count)
        } else {
            Err(count)
        }
    }
}

impl<T> fmt::Debug for Treap<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.root.is_null() {
            return Ok(());
        }

        let mut stack = Vec::new();
        stack.push((self.root, true, 0));
        while let Some((u, first_visit, depth)) = stack.pop() {
            assert_ne!(u, ptr::null_mut());
            if first_visit {
                stack.push((u, false, depth));
                let left = unsafe { &*u }.left;
                let right = unsafe { &*u }.right;
                if !left.is_null() {
                    stack.push((left, true, depth + 1));
                }
                if !right.is_null() {
                    stack.push((right, true, depth + 1));
                }
            } else {
                write!(
                    f,
                    "[{:p}] parent = {:p}, left = {:p}, right = {:p}, x = {:?}, priority = {}, size = {}",
                    u,
                    unsafe { &*u }.parent,
                    unsafe { &*u }.left,
                    unsafe { &*u }.right,
                    unsafe { &*u }.x,
                    unsafe { &*u }.priority,
                    unsafe { &*u }.size
                )?;
                if u != self.root {
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{SSet, Treap};

    #[test]
    fn test_treap_le() {
        let mut treap = Treap::new();
        treap.add(42);
        assert_eq!(treap.le(&41), None);
        assert_eq!(treap.le(&42), Some(&42));
        assert_eq!(treap.le(&43), Some(&42));
    }

    #[test]
    fn test_treap_ge() {
        let mut treap = Treap::new();
        treap.add(42);
        assert_eq!(treap.ge(&41), Some(&42));
        assert_eq!(treap.ge(&42), Some(&42));
        assert_eq!(treap.ge(&43), None);
    }

    #[test]
    fn test_treap_nth() {
        let mut treap = Treap::new();
        treap.add(1);
        treap.add(2);
        treap.add(4);
        treap.add(8);
        assert_eq!(treap.nth(0), Some(&1));
        assert_eq!(treap.nth(1), Some(&2));
        assert_eq!(treap.nth(2), Some(&4));
        assert_eq!(treap.nth(3), Some(&8));
        assert_eq!(treap.nth(4), None);
    }

    #[test]
    fn test_treap_position() {
        let mut treap = Treap::new();
        treap.add(1);
        treap.add(2);
        treap.add(4);
        treap.add(8);
        assert_eq!(treap.position(&0), Err(0));
        assert_eq!(treap.position(&1), Ok(0));
        assert_eq!(treap.position(&2), Ok(1));
        assert_eq!(treap.position(&3), Err(2));
        assert_eq!(treap.position(&4), Ok(2));
        assert_eq!(treap.position(&5), Err(3));
        assert_eq!(treap.position(&6), Err(3));
        assert_eq!(treap.position(&7), Err(3));
        assert_eq!(treap.position(&8), Ok(3));
        assert_eq!(treap.position(&9), Err(4));
    }
}
