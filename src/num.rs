use std::marker::PhantomData;

/// Type level zero value
pub enum Z {}

/// Type level +1
pub struct S<N: Num>(PhantomData<N>);

/// Type level -1
pub struct P<N: Num>(PhantomData<N>);

/// Type level number
pub trait Num {
    /// Next number
    type S: Num;

    /// Previous number
    type P: Num;

    /// Convert to value level
    fn value() -> usize;
}

impl Num for Z {
    type S = S<Z>;
    type P = P<Z>;
    #[inline]
    fn value() -> usize { 0 }
}
impl<N: Num> Num for S<N> {
    type S = S<S<N>>;
    type P = N;
    #[inline]
    fn value() -> usize { N::value() + 1 }
}
impl<N: Num> Num for P<N> {
    type S = N;
    type P = P<P<N>>;
    #[inline]
    fn value() -> usize { N::value() - 1 }
}
