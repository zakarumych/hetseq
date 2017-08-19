use {List, Queue};

#[cfg(not(feature="nightly"))]
use F as Fn;

/// Fold sequence by folding function
pub trait Fold<I, F> {
    fn fold(self, init: I, f: F) -> I;
}

impl<I, F> Fold<I, F> for List<()> {
    fn fold(self, init: I, _: F) -> I { init }
}

impl<I, F> Fold<I, F> for Queue<()> {
    fn fold(self, init: I, _: F) -> I { init }
}

impl<I, X, H, T> Fold<I, X> for List<(H, List<T>)>
    where List<T>: Fold<I, X>,
          X: Fn<(H, I), Output=I>
{
    fn fold(self, init: I, f: X) -> I {
        let List((head, tail)) = self;
        tail.fold(f.call((head, init)), f)
    }
}

impl<I, X, H, T> Fold<I, X> for Queue<(Queue<H>, T)>
    where Queue<H>: Fold<I, X>,
          X: Fn<(T, I), Output=I> + Clone
{
    fn fold(self, init: I, f: X) -> I {
        let Queue((head, tail)) = self;
        f.call((tail, head.fold(init, f.clone())))
    }
}

