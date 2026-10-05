use crate::aggregator::Aggregator;
use alloc::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct DABA<Agg: Aggregator<Value: Clone> + Clone> {
    vals: VecDeque<Agg::Value>,
    aggs: VecDeque<Agg>,
    l: usize,
    r: usize,
    a: usize,
    b: usize,
}

macro_rules! assert_invariants {
    ($s:ident) => {
        let e: usize = s.aggs.len();
        // SAFETY: these are universal invariants of `DABA`
        unsafe {
            ::core::hint::assert_unchecked(e == self.vals.len());
            ::core::hint::assert_unchecked(self.l <= self.r);
            ::core::hint::assert_unchecked(self.r <= self.a);
            ::core::hint::assert_unchecked(self.a <= self.b);
            ::core::hint::assert_unchecked(self.b <= e);
        }
        e
    };
}

impl<Agg: Aggregator<Value: Clone> + Clone> DABA<Agg> {
    pub fn new() {}
}

#[cfg(test)]
mod tests {
    use super::*;
}
