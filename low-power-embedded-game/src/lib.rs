// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    // unimplemented!("implement `fn divmod`");
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // unimplemented!("implement `fn evens`");
    // TODO: remove this; it's only necessary to allow this function to compile
    // before the student has done any work.
    // std::iter::empty()
    let mut idx = 0;
    iter.filter(move |x| {
        let v = idx;
        idx += 1;
        v % 2 == 0
    })
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        // unimplemented!("implement `fn manhattan`")
        let Position(x, y) = self;
        x.abs() + y.abs()
    }
}
