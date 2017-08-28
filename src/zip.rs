use {List, Queue};


pub trait Zip<S> {
    ///
    type Zipped;

    ///
    fn zip(self, S) -> Self::Zipped;
}

impl Zip<List<()>> for List<()> {
    type Zipped = List<()>;

    fn zip(self, _: List<()>) -> List<()> {
        self
    }
}

impl<LH, LT, RH, RT, ZT> Zip<List<(RH, List<RT>)>> for List<(LH, List<LT>)>
    where List<LT>: Zip<List<RT>, Zipped = List<ZT>>
{
    type Zipped = List<((LH, RH), List<ZT>)>;

    fn zip(self, right: List<(RH, List<RT>)>) -> Self::Zipped {
        let List((l_head, l_tail)) = self;
        let List((r_head, r_tail)) = right;

        l_tail.zip(r_tail).push((l_head, r_head))
    }
}

impl Zip<Queue<()>> for Queue<()> {
    type Zipped = Queue<()>;

    fn zip(self, _: Queue<()>) -> Queue<()> {
        self
    }
}

impl<LH, LT, RH, RT, ZH> Zip<Queue<(Queue<RH>, RT)>> for Queue<(Queue<LH>, LT)>
    where Queue<LH>: Zip<Queue<RH>, Zipped = Queue<ZH>>
{
    type Zipped = Queue<(Queue<ZH>, (LT, RT))>;

    fn zip(self, right: Queue<(Queue<RH>, RT)>) -> Self::Zipped {
        let Queue((l_head, l_tail)) = self;
        let Queue((r_head, r_tail)) = right;

        l_head.zip(r_head).push((l_tail, r_tail))
    }
}