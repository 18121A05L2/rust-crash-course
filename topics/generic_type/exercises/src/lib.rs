pub fn first<T1, T2>(t: (T1, T2)) -> T1 {
    t.0
}

pub fn last<T1, T2>(t: (T1, T2)) -> T2 {
    t.1
}

#[derive(Debug)]
pub struct Rectangle<T> {
    pub top: T,
    pub left: T,
    pub width: T,
    pub height: T,
}
