use std::{iter::Map, slice::Iter};


pub trait GetIter<'a> {
    type Item;
    type Iterator: Iterator<Item=Self::Item>;

    fn get_iter(&'a self) -> Self::Iterator;
}

/*
impl<B, I: Iterator, F> Iterator for Map<I, F>
where
    F: FnMut(I::Item) -> B,
{
*/

impl<'a, B: 'a, I: Clone+Iterator+'a, F: Clone+FnMut(I::Item)->B> GetIter<'a> for Map<I, F> {
    type Item=B;
    type Iterator = Map<I, F>;
    fn get_iter(&'a self) -> Self::Iterator {
        self.clone()
    }
}

impl<'a, T: 'a> GetIter<'a> for [T] {
    type Item = &'a T;
    type Iterator = Iter<'a, T>;
    fn get_iter(&'a self) -> Self::Iterator {
        self.iter()
    }
}

impl<'a, const COUNT: usize, T: 'a> GetIter<'a> for [T; COUNT] {
    type Item = &'a T;
    type Iterator = Iter<'a, T>;
    fn get_iter(&'a self) -> Self::Iterator {
        self.iter()
    }
}

fn test() {
    for a in (vec![0,1,2,3]).get_iter() {

    }
}