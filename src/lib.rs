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
/// Arguments are bounded by traits
/// Mainly to use with `Functor` and `Foldable`
#[cfg(not(feature="nightly"))]
#[macro_export]
macro_rules! lambda {
    {let $n:ident = |$($an:ident $(: $at:path)*),+| -> $o:ty { $($s:stmt);* }} => {

        #[derive(Clone, Copy)]
        #[allow(non_camel_case_types)]
        pub struct $n;

        #[allow(non_camel_case_types,unused_mut)]
        impl<$($an: $($at)*,)+> $crate::HetFnOnce<($($an,)*)> for $n {
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

    {let $n:ident$(<$($p:ident $(: $pb:path)*),+>)*($($c:ident: $ct:ty)+) = |$($an:ident $(: $at:path)*),+| -> $o:ty { $($s:stmt);* }} => {

        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct $n<'a, $($($p: 'a),+)*>(::std::marker::PhantomData<&'a ()>, $($ct),+);

        impl<'a, $($($p),+)*> $n<'a, $($($p),+)*>
            where $($($p: 'a $( + $pb,)*)+)*
        {
            pub fn new($($c: $ct)+) -> Self {
                $n(::std::marker::PhantomData, $($c),+)
            }
        }

        #[allow(non_camel_case_types,unused_mut)]
        impl<'a, $($an,)+ $($($p,)+)*> $crate::HetFnOnce<($($an,)*)> for $n<'a, $($($p),+)*>
            where $($($an: $at,)*)+
                  $($($p: 'a $( + $pb,)*)+)*
        {
            type Output = $o;
            fn call_once(self, ($(mut $an,)*): ($($an,)*)) -> $o {
                $crate::HetFn::call(&self, ($($an,)*))
            }
        }

        #[allow(non_camel_case_types,unused_mut)]
        impl<'a, $($an,)+ $($($p,)+)*> $crate::HetFnMut<($($an,)*)> for $n<'a, $($($p),+)*>
            where $($($an: $at,)*)+
                  $($($p: 'a $( + $pb,)*)+)*
        {
            fn call_mut(&mut self, ($(mut $an,)*): ($($an,)*)) -> $o {
                $crate::HetFn::call(self, ($($an,)*))
            }
        }

        #[allow(non_camel_case_types,unused_mut)]
        impl<'a, $($an,)+ $($($p,)+)*> $crate::HetFn<($($an,)*)> for $n<'a, $($($p),+)*>
            where $($($an: $at,)*)+
                  $($($p: 'a $( + $pb,)*)+)*
        {
            fn call(&self, ($(mut $an,)*): ($($an,)*)) -> $o {
                let $n(_, $(ref $c,)+) = *self;
                $($s);*
            }
        }
    };
}


mod by_ref;

#[cfg(not(feature="nightly"))]
mod f;

mod fold;
mod functor;
mod len;
mod list;
mod num;
mod queue;
mod zip;

#[cfg(test)]
mod tests;

#[cfg(feature="nightly")]
mod unsize_iter;

pub use by_ref::ByRef;
#[cfg(not(feature="nightly"))]
pub use f::{HetFnOnce, HetFnMut, HetFn};
pub use fold::Foldable;
pub use functor::Functor;
pub use len::Length;
pub use list::{IntoList, List};
pub use num::{Num, P, S, Z};
pub use queue::{IntoQueue, Queue};
pub use zip::Zip;

#[cfg(feature="nightly")]
pub use unsize_iter::{IntoRefIter, UnsizeRefIter, UnsizeRefIterator};


pub mod prelude;