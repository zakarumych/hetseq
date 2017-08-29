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

impl<LH, LT, RH, RT, ZT> Zip<List<(RH, RT)>> for List<(LH, LT)>
    where LT: Zip<RT, Zipped = ZT>
{
    type Zipped = List<((LH, RH), ZT)>;

    fn zip(self, right: List<(RH, RT)>) -> Self::Zipped {
        let List((l_head, l_tail)) = self;
        let List((r_head, r_tail)) = right;

        List(((l_head, r_head), l_tail.zip(r_tail)))
    }
}

impl Zip<Queue<()>> for Queue<()> {
    type Zipped = Queue<()>;

    fn zip(self, _: Queue<()>) -> Queue<()> {
        self
    }
}

impl<LH, LT, RH, RT, ZH> Zip<Queue<(RH, RT)>> for Queue<(LH, LT)>
    where LH: Zip<RH, Zipped = ZH>
{
    type Zipped = Queue<(ZH, (LT, RT))>;

    fn zip(self, right: Queue<(RH, RT)>) -> Self::Zipped {
        let Queue((l_head, l_tail)) = self;
        let Queue((r_head, r_tail)) = right;

        Queue((l_head.zip(r_head), (l_tail, r_tail)))
    }
}