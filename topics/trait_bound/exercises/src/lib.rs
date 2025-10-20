use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};
use std::marker::Copy;

pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y { x } else { y }
}

pub fn zip<U: Copy + Display + Debug, I: Copy>(a: Vec<U>, b: Vec<I>) -> Vec<(U, I)> {
    let mut v = vec![];
    let len = min(a.len(), b.len());

    for i in 0..len {
        v.push((a[i], b[i]));
        println!("a[i] :: {}", a[i]);
        println!(" a  :: {:?}", a);
    }

    v
}
