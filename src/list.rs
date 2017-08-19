

/// Heterogenous list
/// Supports pushing, splitting to head and tail
/// Mapping and folding
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct List<L>(pub L);
impl List<()> {
    pub fn new() -> List<()> {
        List(())
    }
}

impl<L> List<L> {
    pub fn push<V>(self, value: V) -> List<(V, List<L>)> {
        List((value, self))
    }
}

impl<H, T> List<(H, List<T>)> {
    pub fn head(&self) -> &H {
        let List((ref head, _)) = *self;
        head
    }
    pub fn tail(&self) -> &List<T> {
        let List((_, ref tail)) = *self;
        tail
    }
}