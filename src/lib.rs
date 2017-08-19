#![cfg_attr(feature="nightly", feature(unsize, fn_traits, unboxed_closures))]

#[macro_export]
macro_rules! hlist {
    () => {
        List::new()
    };
    ($head:expr $(, $tail:expr)*) => {
        hlist![$($tail),*].push($head)
    };
}

#[macro_export]
macro_rules! hqueue {
    ($($values:expr),*) => {
        Queue::new()$(.push($values))*
    };
}

#[cfg(not(feature="nightly"))]
mod f;

mod fold;
mod functor;
mod list;
mod num;
mod queue;

#[cfg(test)]
mod tests;

#[cfg(feature="nightly")]
mod unsize_iter;

#[cfg(not(feature="nightly"))]
pub use f::F;
pub use fold::Fold;
pub use functor::Functor;
pub use list::List;
pub use num::{Num, P, S, Z};
pub use queue::Queue;

#[cfg(feature="nightly")]
pub use unsize_iter::{IntoRefIter, UnsizeRefIter, UnsizeRefIterator};

