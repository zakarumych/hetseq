use std::marker::PhantomData;

pub enum Z {}
pub struct S<N: Num>(PhantomData<N>);
pub struct P<N: Num>(PhantomData<N>);

pub trait Num {
    type S: Num;
    type P: Num;
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
