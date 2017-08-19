#![cfg_attr(feature="nightly", feature(unsize, fn_traits, unboxed_closures))]


/// Convenient way to define heterogenous `List`
#[macro_export]
macro_rules! hlist {
    () => {
        List::new()
    };
    ($head:expr $(, $tail:expr)*) => {
        hlist![$($tail),*].push($head)
    };
}

/// Convenient way to define heterogenous `Queue`
#[macro_export]
macro_rules! hqueue {
    ($($values:expr),*) => {
        Queue::new()$(.push($values))*
    };
}

/// This macro can be used to define lambdas with syntax similar to rust's lambdas
/// Arguments are bound by traits
/// Mainly to use with `Functor` and `Foldable`
#[cfg(not(feature="nightly"))]
#[macro_export]
macro_rules! lambda {
    {let $n:ident = |$($(const $anc:ident),* $(mut $anm:ident),* $(: $at:path)*),*| -> $o:ty { $($s:stmt);* }} => {
        #[derive(Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct $n;
        #[allow(non_camel_case_types)]
        impl<$($($anc)*$($anm)*: $($at)*,)*> F<($($($anc)*$($anm)*,)*)> for $n {
            type Output = $o;
            fn call(&self, ($($($anc)*$(mut $anm)*,)*): ($($($anc)*$($anm)*,)*)) -> $o {
                $($s);*
            }
        }
    }
}

/// This macro can be used to define lambdas with syntax similar to rust's lambdas
/// Arguments are bound by traits
/// Mainly to use with `Functor` and `Foldable`
#[cfg(feature="nightly")]
#[macro_export]
macro_rules! lambda {
    {let $n:ident = |$($(const $anc:ident),* $(mut $anm:ident),* $(: $at:path)*),*| -> $o:ty { $($s:stmt);* }} => {
        #[derive(Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct $n;

        #[allow(non_camel_case_types)]
        impl<$($($anc)*$($anm)*: $($at)*,)*> FnOnce<($($($anc)*$($anm)*,)*)> for $n {
            type Output = $o;
            extern "rust-call" fn call_once(self, args: ($($($anc)*$($anm)*,)*)) -> $o {
                self.call(args)
            }
        }

        #[allow(non_camel_case_types)]
        impl<$($($anc)*$($anm)*: $($at)*,)*> FnMut<($($($anc)*$($anm)*,)*)> for $n {
            extern "rust-call" fn call_mut(&mut self, args: ($($($anc)*$($anm)*,)*)) -> $o {
                self.call(args)
            }
        }

        #[allow(non_camel_case_types)]
        impl<$($($anc)*$($anm)*: $($at)*,)*> Fn<($($($anc)*$($anm)*,)*)> for $n {
            extern "rust-call" fn call(&self, ($($($anc)*$(mut $anm)*,)*): ($($($anc)*$($anm)*,)*)) -> $o {
                $($s);*
            }
        }
    }
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
pub use fold::Foldable;
pub use functor::Functor;
pub use list::List;
pub use num::{Num, P, S, Z};
pub use queue::Queue;

#[cfg(feature="nightly")]
pub use unsize_iter::{IntoRefIter, UnsizeRefIter, UnsizeRefIterator};


