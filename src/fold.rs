use {List, Queue};

#[cfg(not(feature="nightly"))]
use F as Fn;

/// Foldable heterogenous list
///
/// # Example
/// ```rust
/// #![cfg_attr(feature="nightly", feature(unsize, fn_traits, unboxed_closures))]
/// #[macro_use]
/// extern crate hetseq;
/// 
/// use hetseq::{Foldable, Functor, Queue};
/// #[cfg(not(feature="nightly"))]
/// use hetseq::F;
/// 
/// use std::fmt::Display;
/// lambda![ let Formatter = |const arg: Display| -> String { format!("{}", arg) } ];
/// lambda![
///     let Extender = |const item, mut extend: Extend<item>| -> extend {
///         extend.extend(::std::iter::once(item));
///         extend
///     }
/// ];
/// fn main() {
///     let queue = hqueue![1, 2.5];
///     let strings = queue.fmap(&Formatter).fold(Vec::new(), &Extender);
///     assert_eq!(strings, ["1", "2.5"]);
/// }
/// ```
pub trait Foldable<I, F> {
    /// fold sequence using `F`unction starting with `I`nit value
    fn fold(self, init: I, f: F) -> I;
}

impl<I, F> Foldable<I, F> for List<()> {
    fn fold(self, init: I, _: F) -> I { init }
}

impl<I, F> Foldable<I, F> for Queue<()> {
    fn fold(self, init: I, _: F) -> I { init }
}

impl<I, X, H, T> Foldable<I, X> for List<(H, List<T>)>
    where List<T>: Foldable<I, X>,
          X: Fn<(H, I), Output=I>
{
    fn fold(self, init: I, f: X) -> I {
        let List((head, tail)) = self;
        tail.fold(f.call((head, init)), f)
    }
}

impl<I, X, H, T> Foldable<I, X> for Queue<(Queue<H>, T)>
    where Queue<H>: Foldable<I, X>,
          X: Fn<(T, I), Output=I> + Clone
{
    fn fold(self, init: I, f: X) -> I {
        let Queue((head, tail)) = self;
        f.call((tail, head.fold(init, f.clone())))
    }
}

