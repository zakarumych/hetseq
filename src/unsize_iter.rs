use std::marker::Unsize;
use {List, Queue};

pub trait UnsizeRefIterator<I: ?Sized> {
    #[inline]
    fn next(&self) -> Option<(&I, &UnsizeRefIterator<I>)>;
    #[inline]
    fn size(&self) -> usize;
}

pub trait UnsizeRefMutIterator<I: ?Sized>: UnsizeRefIterator<I> {
    #[inline]
    fn next_mut(&mut self) -> Option<(&mut I, &mut UnsizeRefIterator<I>)>;
}

impl<I> UnsizeRefIterator<I> for List<()>
    where I: ?Sized
{
    #[inline]
    fn next(&self) -> Option<(&I, &UnsizeRefIterator<I>)> {
        None
    }
    #[inline]
    fn size(&self) -> usize {
        0
    }
}

impl<I> UnsizeRefMutIterator<I> for List<()>
    where I: ?Sized
{
    #[inline]
    fn next_mut(&mut self) -> Option<(&mut I, &mut UnsizeRefIterator<I>)> {
        None
    }
}

impl<I, H, T> UnsizeRefIterator<I> for List<(H, List<T>)>
    where I: ?Sized,
          H: Unsize<I>,
          List<T>: UnsizeRefIterator<I>,
{
    #[inline]
    fn next(&self) -> Option<(&I, &UnsizeRefIterator<I>)> {
        let List((ref head, ref tail)) = *self;
        Some((head, tail))
    }
    #[inline]
    fn size(&self) -> usize {
        1 + self.tail().size()
    }
}

impl<I, H, T> UnsizeRefMutIterator<I> for List<(H, List<T>)>
    where I: ?Sized,
          H: Unsize<I>,
          List<T>: UnsizeRefMutIterator<I>,
{
    #[inline]
    fn next_mut(&mut self) -> Option<(&mut I, &mut UnsizeRefIterator<I>)> {
        let List((ref mut head, ref mut tail)) = *self;
        Some((head, tail))
    }
}

pub struct UnsizeRefIter<'a, I: 'a + ?Sized>(&'a UnsizeRefIterator<I>);

impl<'a, I: 'a> UnsizeRefIter<'a, I>
    where I: ?Sized
{
    fn new(inner: &'a UnsizeRefIterator<I>) -> Self {
        UnsizeRefIter(inner)
    }
}

impl<'a, I: 'a + ?Sized> Iterator for UnsizeRefIter<'a, I> {
    type Item = &'a I;
    fn next(&mut self) -> Option<&'a I> {
        let UnsizeRefIter(ref mut inner) = *self;
        match inner.next() {
            Some((item, next)) => {
                *inner = next;
                Some(item)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.size(), Some(self.0.size()))
    }
}

pub trait IntoRefIter {
    fn into_ref_iter<I: ?Sized>(&self) -> UnsizeRefIter<I> where Self: UnsizeRefIterator<I> + Sized {
        UnsizeRefIter::new(self)
    }
}

impl<Q> IntoRefIter for Queue<Q> {}
impl<L> IntoRefIter for List<L> {}

