use {List, Queue};
pub use num::{Num, P, S, Z};

pub trait Length {
    type Length: Num;
    #[inline]
    fn len() -> usize { Self::Length::value() }
}

impl Length for List<()> {
    type Length = Z;
}

impl<H, T> Length for List<(H, T)>
    where T: Length
{
    type Length = S<<T as Length>::Length>;
}

impl Length for Queue<()> {
    type Length = Z;
}

impl<H, T> Length for Queue<(H, T)>
    where H: Length
{
    type Length = S<<H as Length>::Length>;
}