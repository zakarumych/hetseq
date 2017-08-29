use {List, Queue};

#[cfg(not(feature="nightly"))]
use HetFnMut as FnMut;

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
/// use hetseq::prelude::*;
/// 
/// use std::fmt::Display;
/// lambda![ let Formatter = |arg: Display| -> String { format!("{}", arg) } ];
/// lambda![
///     let Extender = |item, extend: Extend<item>| -> extend {
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

impl<I, F, H, T> Foldable<I, F> for List<(H, T)>
    where T: Foldable<I, F>,
          F: FnMut<(H, I), Output=I>
{
    fn fold(self, init: I, mut f: F) -> I {
        let List((head, tail)) = self;
        tail.fold(f.call_mut((head, init)), f)
    }
}

impl<I, F, H, T> Foldable<I, F> for Queue<(H, T)>
    where H: for<'a> Foldable<I, &'a mut F>,
          F: FnMut<(T, I), Output=I>
{
    fn fold(self, init: I, mut f: F) -> I {
        let Queue((head, tail)) = self;
        let head = head.fold(init, &mut f);
        f.call_mut((tail, head))
    }
}

