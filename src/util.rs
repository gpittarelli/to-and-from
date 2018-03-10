extern crate num;
use num::{PrimInt, Unsigned};

pub fn skip<'a, J, I, N>(i: &mut I, n: N) -> ()
where
    J: 'a,
    I: Iterator<Item = &'a J>,
    N: PrimInt + Unsigned,
{
    for _ in num::range_step(num::zero(), n, num::one()) {
        i.next();
    }
}
