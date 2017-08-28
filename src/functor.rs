use {List, Queue};

#[cfg(not(feature="nightly"))]
use HetFnMut as FnMut;

/// Functor over heterogenous list
///
/// # Example
/// ```rust
/// #![cfg_attr(feature="nightly", feature(unsize, fn_traits, unboxed_closures))]
/// #[macro_use]
/// extern crate hetseq;
/// 
/// use hetseq::{Functor, Queue};
/// #[cfg(not(feature="nightly"))]
/// use hetseq::prelude::*;
/// 
/// use std::fmt::Display;
/// lambda![ let Formatter = |arg: Display| -> String { format!("{}", arg) } ];
/// fn main() {
///     let queue = hqueue![1, 2.5];
///     let strings = queue.fmap(&Formatter);
///     assert_eq!(strings, hqueue!["1".to_owned(), "2.5".to_owned()]);
/// }
/// ```
pub trait Functor<F> {
    /// Result of mapping
    type Output;

    /// Map sequence using `F`unction
    fn fmap(self, F) -> Self::Output;
}

impl<F> Functor<F> for List<()> {
    type Output = List<()>;
    fn fmap(self, _: F) -> List<()> {
        self
    }
}

impl<F> Functor<F> for Queue<()> {
    type Output = Queue<()>;
    fn fmap(self, _: F) -> Queue<()> {
        self
    }
}

impl<F, H, T, O, U> Functor<F> for List<(H, List<T>)>
    where List<T>: Functor<F, Output=List<U>>,
          F: FnMut<(H,), Output=O>,
{
    type Output = List<(O, List<U>)>;
    fn fmap(self, mut f: F) -> Self::Output {
        let List((head, tail)) = self;
        let head = f.call_mut((head,));
        tail.fmap(f).push(head)
    }
}

impl<F, H, T, O, U> Functor<F> for Queue<(Queue<H>, T)>
    where Queue<H>: for<'a> Functor<&'a mut F, Output=Queue<U>>,
          F: FnMut<(T,), Output=O>,
{
    type Output = Queue<(Queue<U>, O)>;
    fn fmap(self, mut f: F) -> Self::Output {
        let Queue((head, tail)) = self;
        head.fmap(&mut f).push(f.call_mut((tail,)))
    }
}
