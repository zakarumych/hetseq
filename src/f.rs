/// Substitution for Fn* traits family in stable and beta channels
/// In nightly consider to use "nightly" feature and Fn* traits family
/// Lambdas doesn't work with heterogenous sequences
/// as they implement Fn* trait for only one parameters set
pub trait F<A> {
    /// Output of the function
    type Output;

    /// Call the function
    fn call(&self, arg: A) -> Self::Output;
}

impl<'a, A, X> F<A> for &'a X
    where X: F<A>
{
    type Output = <X as F<A>>::Output;
    fn call(&self, arg: A) -> Self::Output {
        X::call(self, arg)
    }
}
