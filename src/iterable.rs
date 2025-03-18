use std::{iter::Map, slice::Iter};


pub trait GetIter {
    type Item;
    type Iterator: Iterator<Item=Self::Item>;

    fn get_iter(& self) -> Self::Iterator;
}

impl<'a, A, B: 'a+Iterator<Item=A>, T: ?Sized+GetIter<Item=A, Iterator=B>> GetIter for &T {
    type Item = A;
    type Iterator = B;
    fn get_iter(&self) -> Self::Iterator {
        (*self).get_iter()
    }
}

impl<'a, B: 'a, I: Clone+Iterator+'a, F: 'a+Clone+FnMut(I::Item)->B> GetIter for Map<I, F> {
    type Item=B;
    type Iterator = Map<I, F>;
    fn get_iter(&self) -> Self::Iterator {
        self.clone()
    }
}

impl<'a, T> GetIter for &'a [T] {
    type Item = &'a T;
    type Iterator = Iter<'a, T>;
    fn get_iter(&self) -> Self::Iterator {
        self.iter()
    }
}

impl<'a, const COUNT: usize, T: 'a> GetIter for &'a [T; COUNT] {
    type Item = &'a T;
    type Iterator = Iter<'a, T>;
    fn get_iter(&self) -> Self::Iterator {
        self.iter()
    }
}
