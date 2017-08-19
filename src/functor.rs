use {List, Queue};

#[cfg(not(feature="nightly"))]
use F as Fn;

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
/// use hetseq::F;
/// 
/// use std::fmt::Display;
/// lambda![ let Formatter = |const arg: Display| -> String { format!("{}", arg) } ];
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

impl<X, H, T, O, U> Functor<X> for List<(H, List<T>)>
    where List<T>: Functor<X, Output=List<U>>,
          X: Fn<(H,), Output=O>,
{
    type Output = List<(O, List<U>)>;
    fn fmap(self, f: X) -> Self::Output {
        let List((head, tail)) = self;
        let head = f.call((head,));
        tail.fmap(f).push(head)
    }
}

impl<X, H, T, O, U> Functor<X> for Queue<(Queue<H>, T)>
    where Queue<H>: Functor<X, Output=Queue<U>>,
          X: Fn<(T,), Output=O> + Clone,
{
    type Output = Queue<(Queue<U>, O)>;
    fn fmap(self, f: X) -> Self::Output {
        let Queue((head, tail)) = self;
        head.fmap(f.clone()).push(f.call((tail,)))
    }
}
