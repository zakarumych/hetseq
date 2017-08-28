/// Substitution for FnMut trait family in stable and beta channels
/// In nightly consider to use "nightly" feature and FnMut trait family
pub trait HetFnOnce<A> {
    /// Output of the function
    type Output;

    /// Call the function
    fn call_once(self, arg: A) -> Self::Output;
}

/// Substitution for FnMut trait family in stable and beta channels
/// In nightly consider to use "nightly" feature and FnMut trait family
pub trait HetFnMut<A>: HetFnOnce<A> {
    /// Call the function
    fn call_mut(&mut self, arg: A) -> Self::Output;
}

impl<'a, A, X> HetFnOnce<A> for &'a mut X
    where X: HetFnMut<A>
{
    type Output = <X as HetFnOnce<A>>::Output;
    fn call_once(self, arg: A) -> Self::Output {
        X::call_mut(self, arg)
    }
}

impl<'a, A, X> HetFnMut<A> for &'a mut X
    where X: HetFnMut<A>
{
    fn call_mut(&mut self, arg: A) -> Self::Output {
        X::call_mut(self, arg)
    }
}

/// Substitution for Fn trait family in stable and beta channels
/// In nightly consider to use "nightly" feature and Fn trait family
pub trait HetFn<A>: HetFnMut<A> {
    /// Call the function
    fn call(&self, arg: A) -> Self::Output;
}

impl<'a, A, X> HetFnOnce<A> for &'a X
    where X: HetFn<A>
{
    type Output = <X as HetFnOnce<A>>::Output;
    fn call_once(self, arg: A) -> Self::Output {
        X::call(self, arg)
    }
}

impl<'a, A, X> HetFnMut<A> for &'a X
    where X: HetFn<A>
{
    fn call_mut(&mut self, arg: A) -> Self::Output {
        X::call(self, arg)
    }
}

impl<'a, A, X> HetFn<A> for &'a X
    where X: HetFn<A>
{
    fn call(&self, arg: A) -> Self::Output {
        X::call(self, arg)
    }
}