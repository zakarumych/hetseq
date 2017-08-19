use {List, Queue};

#[cfg(not(feature="nightly"))]
use F as Fn;

pub trait Functor<F> {
    type Output;
    fn fmap(self, F) -> Self::Output;
}

impl<F> Functor<F> for List<()> {
    type Output = List<()>;
    fn fmap(self, _: F) -> List<()> {
        self
    }
}

impl<F> Functor<F> for Queue<()> {
    type Output = Queue<()>;
    fn fmap(self, _: F) -> Queue<()> {
        self
    }
}

impl<X, H, T, O, U> Functor<X> for List<(H, List<T>)>
    where List<T>: Functor<X, Output=List<U>>,
          X: Fn<(H,), Output=O>,
{
    type Output = List<(O, List<U>)>;
    fn fmap(self, f: X) -> Self::Output {
        let List((head, tail)) = self;
        let head = f.call((head,));
        tail.fmap(f).push(head)
    }
}

impl<X, H, T, O, U> Functor<X> for Queue<(Queue<H>, T)>
    where Queue<H>: Functor<X, Output=Queue<U>>,
          X: Fn<(T,), Output=O> + Clone,
{
    type Output = Queue<(Queue<U>, O)>;
    fn fmap(self, f: X) -> Self::Output {
        let Queue((head, tail)) = self;
        head.fmap(f.clone()).push(f.call((tail,)))
    }
}
