use {List, Queue};

pub trait ByRef<'a> {
    ///
    type Refs: 'a;

    ///
    fn by_ref(&'a self) -> Self::Refs;
}

impl<'a> ByRef<'a> for List<()> {
    type Refs = List<()>;

    fn by_ref(&'a self) -> Self::Refs {
        List::new()
    }
}

impl<'a, H: 'a, T: 'a, R: 'a> ByRef<'a> for List<(H, List<T>)>
    where List<T>: ByRef<'a, Refs=List<R>>
{
    type Refs = List<(&'a H, List<R>)>;

    fn by_ref(&'a self) -> Self::Refs {
        let List((ref head, ref tail)) = *self;
        tail.by_ref().push(head)
    }
}

impl<'a> ByRef<'a> for Queue<()> {
    type Refs = Queue<()>;

    fn by_ref(&'a self) -> Self::Refs {
        Queue::new()
    }
}

impl<'a, H: 'a, T: 'a, R: 'a> ByRef<'a> for Queue<(Queue<H>, T)>
    where Queue<H>: ByRef<'a, Refs=Queue<R>>
{
    type Refs = Queue<(Queue<R>, &'a T)>;

    fn by_ref(&'a self) -> Self::Refs {
        let Queue((ref head, ref tail)) = *self;
        head.by_ref().push(tail)
    }
}
