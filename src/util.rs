extern crate num;
use num::{PrimInt, Unsigned};
use std::path::Path;

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

pub fn basename(path: &str) -> Option<&str> {
    Path::new(path).file_name().and_then(|os| os.to_str())
}
