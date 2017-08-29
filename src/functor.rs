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

impl<'a, F> Functor<F> for &'a List<()> {
    type Output = List<()>;
    fn fmap(self, _: F) -> List<()> {
        List::new()
    }
}

impl<F> Functor<F> for Queue<()> {
    type Output = Queue<()>;
    fn fmap(self, _: F) -> Queue<()> {
        self
    }
}

impl<'a, F> Functor<F> for &'a Queue<()> {
    type Output = Queue<()>;
    fn fmap(self, _: F) -> Queue<()> {
        Queue::new()
    }
}

impl<F, H, T, O, U> Functor<F> for List<(H, T)>
    where T: Functor<F, Output=U>,
          F: FnMut<(H,), Output=O>,
{
    type Output = List<(O, U)>;
    fn fmap(self, mut f: F) -> Self::Output {
        let List((head, tail)) = self;
        let head = f.call_mut((head,));
        List((head, tail.fmap(f)))
    }
}

impl<'a, F, H, T, O, U> Functor<F> for &'a mut List<(H, T)>
    where &'a mut T: Functor<F, Output=U>,
          F: FnMut<(&'a mut H,), Output=O>,
{
    type Output = List<(O, U)>;
    fn fmap(self, mut f: F) -> Self::Output {
        let List((ref mut head, ref mut tail)) = *self;
        let head = f.call_mut((head,));
        List((head, tail.fmap(f)))
    }
}

impl<'a, F, H, T, O, U> Functor<F> for &'a List<(H, T)>
    where &'a T: Functor<F, Output=U>,
          F: FnMut<(&'a H,), Output=O>,
{
    type Output = List<(O, U)>;
    fn fmap(self, mut f: F) -> Self::Output {
        let List((ref head, ref tail)) = *self;
        let head = f.call_mut((head,));
        List((head, tail.fmap(f)))
    }
}

impl<F, H, T, O, U> Functor<F> for Queue<(H, T)>
    where H: for<'a> Functor<&'a mut F, Output=U>,
          F: FnMut<(T,), Output=O>,
{
    type Output = Queue<(U, O)>;
    fn fmap(self, mut f: F) -> Self::Output {
        let Queue((head, tail)) = self;
        Queue((head.fmap(&mut f), f.call_mut((tail,))))
    }
}


impl<'a, F, H, T, O, U> Functor<F> for &'a mut Queue<(H, T)>
    where &'a mut H: for<'b> Functor<&'b mut F, Output=U>,
          F: FnMut<(&'a mut T,), Output=O>,
{
    type Output = Queue<(U, O)>;
    fn fmap(self, mut f: F) -> Self::Output {
        let Queue((ref mut head, ref mut tail)) = *self;
        Queue((head.fmap(&mut f), f.call_mut((tail,))))
    }
}


impl<'a, F, H, T, O, U> Functor<F> for &'a Queue<(H, T)>
    where &'a H: for<'b> Functor<&'b mut F, Output=U>,
          F: FnMut<(&'a T,), Output=O>,
{
    type Output = Queue<(U, O)>;
    fn fmap(self, mut f: F) -> Self::Output {
        let Queue((ref head, ref tail)) = *self;
        Queue((head.fmap(&mut f), f.call_mut((tail,))))
    }
}
