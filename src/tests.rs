
use {Foldable, Functor, List, Queue};
#[cfg(feature="nightly")]
use IntoRefIter;
#[cfg(feature="nightly")]
use std::fmt::Display;

#[cfg(not(feature="nightly"))]
mod functions {
use std::fmt::Display;
use F;

lambda!{
    let Formatter = |const arg: Display| -> String {
        format!("{}", arg)
    }
}

lambda!{
    let Extender = |const item, mut extend: Extend<item>| -> extend {
        extend.extend(::std::iter::once(item));
        extend
    }
}

}

#[cfg(feature="nightly")]
mod functions {
use super::*;

#[derive(Clone, Copy)]
pub struct Formatter;
impl<A: Display> FnOnce<(A,)> for Formatter {
    type Output = String;
    extern "rust-call" fn call_once(self, args: (A,)) -> String {
        self.call(args)
    }
}
impl<A: Display> FnMut<(A,)> for Formatter {
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> String {
        self.call(args)
    }
}
impl<A: Display> Fn<(A,)> for Formatter {
    extern "rust-call" fn call(&self, (arg,): (A,)) -> String {
        format!("{}", arg)
    }
}

#[derive(Clone, Copy)]
pub struct Extender;
impl<I, E> FnOnce<(I, E)> for Extender
    where E: Extend<I>
{
    type Output = E;
    extern "rust-call" fn call_once(self, args: (I, E)) -> E {
        self.call(args)
    }
}
impl<I, E> FnMut<(I, E)> for Extender
    where E: Extend<I>
{
    extern "rust-call" fn call_mut(&mut self, args: (I, E)) -> E {
        self.call(args)
    }
}
impl<I, E> Fn<(I, E)> for Extender
    where E: Extend<I>
{
    extern "rust-call" fn call(&self, (item, mut extend): (I, E)) -> E {
        extend.extend(::std::iter::once(item));
        extend
    }
}

}

use self::functions::{Formatter, Extender};

const EXPECT: [&'static str; 3] = ["1", "2.5", "qwe"];

#[test]
fn test_macro() {
    let list_macro = hlist![1, 2.5, "qwe"];
    let list_manual = List::new().push("qwe").push(2.5).push(1);
    assert_eq!(list_macro, list_manual);

    let queue_macro = hqueue![1, 2.5, "qwe"];
    let queue_manual = Queue::new().push(1).push(2.5).push("qwe");
    assert_eq!(queue_macro, queue_manual);
}

#[test]
fn test_map_fold() {
    let queue = Queue::new().push(1).push(2.5).push("qwe");
    let queue = queue.fmap(Formatter).fold(Vec::new(), Extender);
    assert_eq!(queue, EXPECT);

    let list = List::new().push("qwe").push(2.5).push(1);
    let list = list.fmap(Formatter).fold(Vec::new(), Extender);
    assert_eq!(list, EXPECT);
}

#[cfg(feature="nightly")]
#[test]
fn test_unsize_ref_iter() {
    let list = List::new().push("qwe").push(2.5).push(1);
    let list = list
        .into_ref_iter::<ToString>()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    assert_eq!(list, EXPECT);
}
