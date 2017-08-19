
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
    pub fn head(&self) -> &Queue<H> {
        &(self.0).0
    }
    pub fn tail(&self) -> &T {
        &(self.0).1
    }
}