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
/// Built-in lambdas doesn't work with heterogenous sequences
/// as they implement Fn* traits only for one parameters set
#[cfg(not(feature="nightly"))]
#[macro_export]
macro_rules! lambda {
    {let $n:ident = |$($an:ident $(: $at:path)*),*| -> $o:ty { $($s:stmt);* }} => {

        #[derive(Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct $n;

        #[allow(non_camel_case_types,unused_mut)]
        impl<$($an: $($at)*,)*> $crate::HetFnOnce<($($an,)*)> for $n {
            type Output = $o;
            fn call_once(self, ($(mut $an,)*): ($($an,)*)) -> $o {
                Self::call(&self, ($($an,)*))
            }
        }
        
        #[allow(non_camel_case_types,unused_mut)]
        impl<$($an: $($at)*,)*> $crate::HetFnMut<($($an,)*)> for $n {
            fn call_mut(&mut self, ($(mut $an,)*): ($($an,)*)) -> $o {
                Self::call(&self, ($($an,)*))
            }
        }

        #[allow(non_camel_case_types,unused_mut)]
        impl<$($an: $($at)*,)*> $crate::HetFn<($($an,)*)> for $n {
            fn call(&self, ($(mut $an,)*): ($($an,)*)) -> $o {
                $($s);*
            }
        }
    };

    {let $n:ident($($ctx:ident),+) = |$($an:ident $(: $at:path)*),*| -> $o:ty { $($s:stmt);* }} => {

        #[derive(Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct $n<$(($($ctx),+))*>($(($($ctx),+))*);

        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> $crate::HetFnOnce<($($an,)*)> for $n<$(($($ctx),+))*><$(($($ctx),+))*> {
            type Output = $o;
            fn call_once(self, ($(mut $an,)*): ($($an,)*)) -> $o {
                Self::call(&self, ($($an,)*))
            }
        }
        
        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> $crate::HetFnMut<($($an,)*)> for $n<$(($($ctx),+))*> {
            fn call_mut(&mut self, ($(mut $an,)*): ($($an,)*)) -> $o {
                Self::call(&self, ($($an,)*))
            }
        }

        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> $crate::HetFn<($($an,)*)> for $n<$(($($ctx),+))*> {
            fn call(&self, ($(mut $an,)*): ($($an,)*)) -> $o {
                $($s);*
            }
        }
    };


    {let mut $n:ident($($ctx:ident),+) = |$($an:ident $(: $at:path)*),*| -> $o:ty { $($s:stmt);* }} => {

        #[derive(Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct $n<$(($($ctx),+))*>($(($($ctx),+))*);

        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> $crate::HetFnOnce<($($an,)*)> for $n<$(($($ctx),+))*> {
            type Output = $o;
            fn call_once(mut self, ($(mut $an,)*): ($($an,)*)) -> $o {
                Self::call_mut(&mut self)
            }
        }
        
        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> $crate::HetFnMut<($($an,)*)> for $n<$(($($ctx),+))*> {
            fn call_mut(&mut self, ($(mut $an,)*): ($($an,)*)) -> $o {
                $($s);*
            }
        }
    }
}

/// This macro can be used to define lambdas with syntax similar to rust's lambdas
/// Arguments are bound by traits
/// Mainly to use with `Functor` and `Foldable`
/// Built-in lambdas doesn't work with heterogenous sequences
/// as they implement Fn* traits only for one parameters set
#[cfg(feature="nightly")]
#[macro_export]
macro_rules! lambda {
    {let $n:ident = |$($an:ident $(: $at:path)*),*| -> $o:ty { $($s:stmt);* }} => {

        #[derive(Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct $n<$(($($ctx),+))*>($(($($ctx),+))*);

        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> FnOnce<($($an,)*)> for $n<$(($($ctx),+))*> {
            type Output = $o;
            extern "rust-call" fn call_once(self, ($(mut $an,)*): ($($an,)*)) -> $o {
                Self::call(&self, ($($an,)*))
            }
        }
        
        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> FnMut<($($an,)*)> for $n<$(($($ctx),+))*> {
            extern "rust-call" fn call_mut(&mut self, ($(mut $an,)*): ($($an,)*)) -> $o {
                Self::call(&self, ($($an,)*))
            }
        }

        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> Fn<($($an,)*)> for $n<$(($($ctx),+))*> {
            extern "rust-call" fn call(&self, ($(mut $an,)*): ($($an,)*)) -> $o {
                $($s);*
            }
        }
    };

    {let $n:ident($($ctx:ident),+) = |$($an:ident $(: $at:path)*),*| -> $o:ty { $($s:stmt);* }} => {

        #[derive(Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct $n<$(($($ctx),+))*>($(($($ctx),+))*);

        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> FnOnce<($($an,)*)> for $n<$(($($ctx),+))*> {
            type Output = $o;
            extern "rust-call" fn call_once(self, ($(mut $an,)*): ($($an,)*)) -> $o {
                Self::call(&self, ($($an,)*))
            }
        }
        
        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> FnMut<($($an,)*)> for $n<$(($($ctx),+))*> {
            extern "rust-call" fn call_mut(&mut self, ($(mut $an,)*): ($($an,)*)) -> $o {
                Self::call(&self, ($($an,)*))
            }
        }

        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> Fn<($($an,)*)> for $n<$(($($ctx),+))*> {
            extern "rust-call" fn call(&self, ($(mut $an,)*): ($($an,)*)) -> $o {
                $($s);*
            }
        }
    };


    {let mut $n:ident($($ctx:ident),+) = |$($an:ident $(: $at:path)*),*| -> $o:ty { $($s:stmt);* }} => {

        #[derive(Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct $n<$(($($ctx),+))*>($(($($ctx),+))*);

        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> FnOnce<($($an,)*)> for $n<$(($($ctx),+))*> {
            type Output = $o;
            extern "rust-call" fn call_once(mut self, ($(mut $an,)*): ($($an,)*)) -> $o {
                Self::call_mut(&mut self)
            }
        }
        
        #[allow(non_camel_case_types)]
        impl<$($an: $($at)*,)* $(($(,$ctx)+))*> FnMut<($($an,)*)> for $n<$(($($ctx),+))*> {
            extern "rust-call" fn call_mut(&mut self, ($(mut $an,)*): ($($an,)*)) -> $o {
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
pub use f::{HetFnOnce, HetFnMut, HetFn};
pub use fold::Foldable;
pub use functor::Functor;
pub use list::List;
pub use num::{Num, P, S, Z};
pub use queue::Queue;

#[cfg(feature="nightly")]
pub use unsize_iter::{IntoRefIter, UnsizeRefIter, UnsizeRefIterator};


pub mod prelude;