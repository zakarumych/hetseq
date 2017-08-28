use {IntoList, List};

/// Heterogenous queue
/// Supports pushing, splitting to init and last
/// Mapping and folding
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Queue<Q>(pub Q);

impl Queue<()> {
    pub fn new() -> Queue<()> {
        Queue(())
    }
}

impl<Q> Queue<Q> {
    pub fn push<V>(self, value: V) -> Queue<(Queue<Q>, V)> {
        Queue((self, value))
    }
}

impl<H, T> Queue<(Queue<H>, T)> {
    pub fn init(&self) -> &Queue<H> {
        &(self.0).0
    }
    pub fn last(&self) -> &T {
        &(self.0).1
    }
}

pub trait IntoQueue {
    ///
    type Queue;
    ///
    fn into_queue(self) -> Self::Queue;
}

pub trait IntoListImpl<L> {
    type List;
    fn into_list_impl(self, L) -> Self::List;
}

impl<H, T, L> IntoListImpl<List<L>> for Queue<(H, T)>
    where H: IntoListImpl<List<(T, List<L>)>>
{
    type List = H::List;

    fn into_list_impl(self, list: List<L>) -> Self::List {
        let Queue((head, tail)) = self;
        let list = list.push(tail);
        head.into_list_impl(list)
    }
}

impl<Q> IntoList for Q
    where Q: IntoListImpl<List<()>>
{
    type List = <Q as IntoListImpl<List<()>>>::List;
    fn into_list(self) -> Self::List {
        self.into_list_impl(List::new())
    }
}