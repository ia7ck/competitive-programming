use join::Join;
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use std::{
    alloc,
    cmp::{self, Ordering},
    fmt::{self, Formatter},
    ptr,
};

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
            rng: SmallRng::seed_from_u64(122333),
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
        if p == ptr::null_mut() {
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
        if b == ptr::null_mut() {
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
        if p == ptr::null_mut() {
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
        if b == ptr::null_mut() {
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
        if u == ptr::null_mut() {
            0
        } else {
            unsafe { &*u }.size
        }
    }

    fn nth_inner(&self, n: usize, u: *mut Node<T>) -> Option<&T> {
        debug_assert_ne!(u, ptr::null_mut());
        let left = unsafe { &*u }.left;
        let left_size = Self::node_size(left);
        if left_size == n {
            Some(&unsafe { &*u }.x)
        } else if left_size > n {
            debug_assert_ne!(left, ptr::null_mut());
            self.nth_inner(n, left)
        } else {
            let right = unsafe { &*u }.right;
            debug_assert_ne!(right, ptr::null_mut());
            self.nth_inner(n - left_size - 1, right)
        }
    }
}

trait SSet<T> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: &T) -> bool;
    fn find(&self, x: &T) -> Option<&T>;
    fn nth(&self, n: usize) -> Option<&T>;
}

impl<T> Treap<T>
where
    T: cmp::Ord,
{
    fn find_last(&self, x: &T) -> *mut Node<T> {
        let mut w = self.root;
        let mut prev = ptr::null_mut();
        while w != ptr::null_mut() {
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
        let u = if p == ptr::null_mut() {
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
        if p != ptr::null_mut() && unsafe { &*p }.x.eq(&x) {
            return false;
        }

        let u = self.add_child(p, x);
        // bubble up
        loop {
            let p = unsafe { &*u }.parent;
            if p == ptr::null_mut() {
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
        if unsafe { &*u }.parent == ptr::null_mut() {
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
        if u == ptr::null_mut() {
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
            if left == ptr::null_mut() && right == ptr::null_mut() {
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
            if left == ptr::null_mut() {
                self.rotate_left(u);
            } else if right == ptr::null_mut() {
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

    fn find(&self, x: &T) -> Option<&T> {
        let mut w = self.root;
        let mut z = ptr::null_mut();
        loop {
            if w == ptr::null_mut() {
                break;
            }
            let y = &unsafe { &*w }.x;
            match x.cmp(y) {
                Ordering::Less => {
                    z = w;
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

        if z == ptr::null_mut() {
            None
        } else {
            Some(&unsafe { &*z }.x)
        }
    }

    // 0-indexed
    fn nth(&self, n: usize) -> Option<&T> {
        if n >= self.size() {
            None
        } else {
            self.nth_inner(n, self.root)
        }
    }
}

impl<T> fmt::Debug for Treap<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.root == ptr::null_mut() {
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
                if left != ptr::null_mut() {
                    stack.push((left, true, depth + 1));
                }
                if right != ptr::null_mut() {
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

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        names: [String; n],
    };

    let mut set = Treap::new();
    for i in 0..n {
        set.add((names[i].clone(), i));
    }

    for _ in 0..q {
        input! {
            x: usize,
            t: String,
        };
        let (s, i) = set.nth(x - 1).unwrap().clone();
        set.remove(&(s, i));
        set.add((t, i));
    }
    let mut ans = vec![String::new(); n];
    for i in 0..n {
        let (s, j) = set.nth(i).unwrap().clone();
        ans[j] = s;
    }
    println!("{}", ans.iter().join(" "));
}
