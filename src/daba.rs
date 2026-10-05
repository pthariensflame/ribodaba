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
    ($s:expr) => {{
        let s: &DABA<_> = $s;
        let e: usize = s.aggs.len();
        // SAFETY: these are universal invariants of `DABA`
        unsafe {
            ::core::hint::assert_unchecked(e == s.vals.len());
            ::core::hint::assert_unchecked(s.l <= s.r);
            ::core::hint::assert_unchecked(s.r <= s.a);
            ::core::hint::assert_unchecked(s.a <= s.b);
            ::core::hint::assert_unchecked(s.b <= e);
        }
        e
    }};
}

impl<Agg: Aggregator<Value: Clone> + Clone> DABA<Agg> {
    #[inline]
    pub fn new(expected_window_size: usize) -> Self {
        Self {
            vals: VecDeque::with_capacity(expected_window_size),
            aggs: VecDeque::with_capacity(expected_window_size),
            l: 0,
            r: 0,
            a: 0,
            b: 0,
        }
    }

    #[inline]
    fn agg_f(&self) -> Agg {
        assert_invariants!(self);

        if self.b == 0 {
            Agg::empty()
        } else {
            // SAFETY: there must be at least one element if b > 0
            unsafe { self.aggs.front().unwrap_unchecked().clone() }
        }
    }

    #[inline]
    fn agg_b(&self) -> Agg {
        let e = assert_invariants!(self);

        if self.b == e {
            Agg::empty()
        } else {
            // SAFETY: there must be at least one element if b < e
            unsafe { self.aggs.back().unwrap_unchecked().clone() }
        }
    }

    #[inline]
    fn agg_l(&self) -> Agg {
        assert_invariants!(self);

        if self.l == self.r {
            Agg::empty()
        } else {
            // SAFETY: there must be at least one element if l < r
            unsafe { self.aggs.get(self.l).unwrap_unchecked().clone() }
        }
    }

    #[inline]
    fn agg_r(&self) -> Agg {
        assert_invariants!(self);

        if self.r == self.a {
            Agg::empty()
        } else {
            // SAFETY: there must be at least one element if r < a, and a must be strictly positive
            unsafe {
                self.aggs
                    .get(self.a.unchecked_sub(1))
                    .unwrap_unchecked()
                    .clone()
            }
        }
    }

    #[inline]
    fn agg_a(&self) -> Agg {
        assert_invariants!(self);

        if self.a == self.b {
            Agg::empty()
        } else {
            // SAFETY: there must be at least one element if a < b
            unsafe { self.aggs.get(self.a).unwrap_unchecked().clone() }
        }
    }

    #[inline]
    pub fn current_summary(&self) -> Agg::Summary {
        self.agg_f().merge(self.agg_b()).summarize()
    }

    #[inline]
    pub fn push(&mut self, val: Agg::Value) {
        assert_invariants!(self);

        self.aggs
            .push_back(self.agg_b().incorporate_after(val.clone()));
        self.vals.push_back(val);
    }

    #[inline]
    pub fn discard(&mut self) {
        let e = assert_invariants!(self);
        assert!(e > 0, "Attempted to discard from an empty window");

        // SAFETY: queues are known to be nonempty
        unsafe {
            self.vals.pop_front().unwrap_unchecked();
            self.aggs.pop_front().unwrap_unchecked();
        }

        self.fixup();
    }

    fn fixup(&mut self) {
        let e = assert_invariants!(self);
        if e == 0 {
            return;
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
