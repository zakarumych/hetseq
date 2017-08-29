
use List;

use rayon::iter::{Chain, IntoParallelIterator, ParallelIterator};

impl<H, T, I> IntoParallelIterator for List<(H, T)>
    where H: IntoParallelIterator<Item=I>,
          T: IntoParallelIterator<Item=I>,
          I: Send,
{
    type Iter = Chain<H::Iter, T::Iter>;
    type Item = I;
    fn into_par_iter(self) -> Chain<H::Iter, T::Iter> {
        let List((head, tail)) = self;
        head.into_par_iter().chain(tail)
    }
}

impl<H, I> IntoParallelIterator for List<(H, List<()>)>
    where H: IntoParallelIterator<Item=I>,
          I: Send,
{
    type Iter = H::Iter;
    type Item = I;
    fn into_par_iter(self) -> H::Iter {
        let List((head, List(()))) = self;
        head.into_par_iter()
    }
}