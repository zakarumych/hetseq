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


impl<E, H, T, Z> Try<E> for List<(Result<H, E>, T)>
    where T: Try<E, Ok=Z>
{
    type Ok = List<(H, Z)>;

    fn try(self) -> Result<List<(H, Z)>, E> {
        let List((head, tail)) = self;
        let head = head?;
        let tail = tail.try()?;
        Ok(List((head, tail)))
    }
}

impl<E> Try<E> for Queue<()> {
    type Ok = Queue<()>;

    fn try(self) -> Result<Queue<()>, E> {
        Ok(self)
    }
}


impl<E, H, T, Z> Try<E> for Queue<(H, Result<T, E>)>
    where H: Try<E, Ok=Z>
{
    type Ok = Queue<(Z, T)>;

    fn try(self) -> Result<Queue<(Z, T)>, E> {
        let Queue((head, tail)) = self;
        let head = head.try()?;
        let tail = tail?;
        Ok(Queue((head, tail)))
    }
}
