
use {Foldable, Functor, List, Queue};
#[cfg(feature="nightly")]
use IntoRefIter;
#[cfg(feature="nightly")]
use std::fmt::Display;

#[cfg(not(feature="nightly"))]
mod functions {
use std::fmt::Display;
use std::ops::Add;
use HetFn;

lambda!{
    let Formatter = |arg: Display| -> String {
        format!("{}", arg)
    }
}

lambda!{
    let Extender = |item, extend: Extend<item>| -> extend {
        extend.extend(::std::iter::once(item));
        extend
    }
}

lambda!{
    let Contextual(x: Copy) = |value: Add<x>| -> value::Output {
        value + *x
    }
}

}

#[cfg(feature="nightly")]
mod functions {
use super::*;

lambda!{
    let Formatter = |arg: Display| -> String {
        format!("{}", arg)
    }
}

lambda!{
    let Extender = |item, extend: Extend<item>| -> extend {
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
