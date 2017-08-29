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

impl<'a, H: 'a, T: 'a, R: 'a> ByRef<'a> for List<(H, T)>
    where T: ByRef<'a, Refs=R>
{
    type Refs = List<(&'a H, R)>;

    fn by_ref(&'a self) -> Self::Refs {
        let List((ref head, ref tail)) = *self;
        List((head, tail.by_ref()))
    }
}

impl<'a> ByRef<'a> for Queue<()> {
    type Refs = Queue<()>;

    fn by_ref(&'a self) -> Self::Refs {
        Queue::new()
    }
}

impl<'a, H: 'a, T: 'a, R: 'a> ByRef<'a> for Queue<(H, T)>
    where H: ByRef<'a, Refs=R>
{
    type Refs = Queue<(R, &'a T)>;

    fn by_ref(&'a self) -> Self::Refs {
        let Queue((ref head, ref tail)) = *self;
        Queue((head.by_ref(), tail))
    }
}
