#![feature(test)]

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::borrow::Cow;
use std::ptr::eq;

fn min_sum_cow(min: i32, v: &mut Cow<[i32]>) {
    let sum: i32 = v.iter().sum();
    if sum < min {
        v.to_mut().push(min - sum);
    }
}

fn min_sum_refcell(min: i32, v: &RefCell<Vec<i32>>) {
    let sum: i32 = v.borrow().iter().sum();
    if sum < min {
        v.borrow_mut().push(min - sum);
    }
}

fn min_sum_cell(min: i32, v: &Cell<Vec<i32>>) {
    let mut vec = v.take();
    let sum: i32 = vec.iter().sum();
    if sum < min {
        vec.push(min - sum);
    }
    v.set(vec);
}
fn length(s: String) -> usize {
    s.len()
}
fn rc_length(s: Rc<String>) -> usize {
    s.len() // calls to the wrapped object require no additions
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cloning() {
        let s = "abcdef".to_owned();
        assert_eq!(length(s), 6);
        // s is now "gone", we can't use it anymore
        // therefore we can't use it in a loop either!
        // ... unless we clone s - at a cost! (see benchmark)
        let s = "abcdef".to_owned();

        for _ in 0..10 {
            // clone is typically an expensive deep copy
            assert_eq!(length(s.clone()), 6);
        }
    }

    extern crate test;
    use std::rc::Rc;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_string_clone(b: &mut Bencher) {
        let s: String = (0..100_000).map(|_| 'a').collect();
        b.iter(|| {
            black_box(length(s.clone()));
        });
    }

    #[test]
    fn refcounting() {
        let s = Rc::new("abcdef".to_owned());
        // we can clone Rc (reference counters) with low cost
        assert_eq!(rc_length(s.clone()), 6);

        for _ in 0..10 {
            // clone is typically an expensive deep copy
            assert_eq!(rc_length(s.clone()), 6);
        }
    }

    #[bench]
    fn bench_string_rc(b: &mut Bencher) {
        let s: String = (0..100_000).map(|_| 'a').collect();
        let rc_s = Rc::new(s);
        b.iter(|| {
            black_box(rc_length(rc_s.clone()));
        });
    }

    #[bench]
    fn bench_regular_push(b: &mut Bencher) {
        let mut v = vec![];
        b.iter(|| {
            for _ in 0..1_000 {
                v.push(10);
            }
        });
    }

    #[bench]
    fn bench_refcell_push(b: &mut Bencher) {
        let v = RefCell::new(vec![]);
        b.iter(|| {
            for _ in 0..1_000 {
                v.borrow_mut().push(10);
            }
        });
    }

    #[bench]
    fn bench_cell_push(b: &mut Bencher) {
        let v = Cell::new(vec![]);
        b.iter(|| {
            for _ in 0..1_000 {
                let mut vec = v.take();
                vec.push(10);
                v.set(vec);
            }
        });
    }
}
