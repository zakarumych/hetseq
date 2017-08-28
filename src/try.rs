use {List, Queue};


pub trait Try<E> {
    /// Sequence of `Ok` `Result`s
    type Ok;

    fn try(self) -> Result<Self::Ok, E>;
}

impl<E> Try<E> for List<()> {
    type Ok = List<()>;

    fn try(self) -> Result<List<()>, E> {
        Ok(self)
    }
}


impl<E, H, T, Z> Try<E> for List<(Result<H, E>, List<T>)>
    where List<T>: Try<E, Ok=List<Z>>
{
    type Ok = List<(H, List<Z>)>;

    fn try(self) -> Result<List<(H, List<Z>)>, E> {
        let List((head, tail)) = self;
        let head = head?;
        let tail = tail.try()?;
        Ok(tail.push(head))
    }
}

impl<E> Try<E> for Queue<()> {
    type Ok = Queue<()>;

    fn try(self) -> Result<Queue<()>, E> {
        Ok(self)
    }
}


impl<E, H, T, Z> Try<E> for Queue<(Queue<H>, Result<T, E>)>
    where Queue<H>: Try<E, Ok=Queue<Z>>
{
    type Ok = Queue<(Queue<Z>, T)>;

    fn try(self) -> Result<Queue<(Queue<Z>, T)>, E> {
        let Queue((head, tail)) = self;
        let head = head.try()?;
        let tail = tail?;
        Ok(head.push(tail))
    }
}
