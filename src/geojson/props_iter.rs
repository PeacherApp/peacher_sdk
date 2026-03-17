use std::iter::FusedIterator;

use crate::geojson::{GeoJsonFeature, GeoJsonFeatureCollection};

pub struct RefPropsIter<'a, T> {
    inner: InnerPropsIter<'a, T>,
}

impl<'a, T> RefPropsIter<'a, T> {
    pub(super) fn one(prop: &'a T) -> Self {
        Self {
            inner: InnerPropsIter::One(std::iter::once(prop)),
        }
    }
    pub(super) fn many(collection: &'a GeoJsonFeatureCollection<T>) -> Self {
        Self {
            inner: InnerPropsIter::Many(collection.features.iter()),
        }
    }
}
// implemented as many methods as possible for this type
impl<'a, T> Iterator for RefPropsIter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            InnerPropsIter::One(value) => value.next(),
            InnerPropsIter::Many(iter) => {
                let value = iter.next()?;
                Some(&value.properties)
            }
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner {
            InnerPropsIter::Many(m) => m.size_hint(),
            InnerPropsIter::One(m) => m.size_hint(),
        }
    }
    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        match self.inner {
            InnerPropsIter::Many(m) => m.count(),
            InnerPropsIter::One(m) => m.count(),
        }
    }
    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match &mut self.inner {
            InnerPropsIter::Many(m) => m.nth(n).map(|p| &p.properties),
            InnerPropsIter::One(m) => m.nth(n),
        }
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        // this is the impl on slice::Iter
        self.next_back()
    }
    #[inline]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        match self.inner {
            InnerPropsIter::Many(m) => m.fold(init, |init, val| f(init, &val.properties)),
            InnerPropsIter::One(m) => m.fold(init, f),
        }
    }
}

impl<'a, T> ExactSizeIterator for RefPropsIter<'a, T> {
    fn len(&self) -> usize {
        match &self.inner {
            InnerPropsIter::Many(m) => m.len(),
            InnerPropsIter::One(m) => m.len(),
        }
    }
    //todo: impl is_empty when stabilized
}

impl<'a, T> DoubleEndedIterator for RefPropsIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            InnerPropsIter::Many(m) => {
                let val = m.next_back()?;
                Some(&val.properties)
            }
            InnerPropsIter::One(m) => m.next_back(),
        }
    }
}
impl<'a, T> FusedIterator for RefPropsIter<'a, T> {}

enum InnerPropsIter<'a, T> {
    One(std::iter::Once<&'a T>),
    Many(std::slice::Iter<'a, GeoJsonFeature<T>>),
}
