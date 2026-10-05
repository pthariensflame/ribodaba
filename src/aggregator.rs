/// A way of summarizing a sequence of elements; a monoid with an injection and a projection.
pub trait Aggregator: Sized {
    type Value;
    type Summary;

    fn from_value(val: Self::Value) -> Self;
    fn summarize(self) -> Self::Summary;

    fn empty() -> Self;
    fn merge(self, other: Self) -> Self;

    #[inline]
    fn incorporate_before(self, val: Self::Value) -> Self {
        Self::from_value(val).merge(self)
    }

    #[inline]
    fn incorporate_after(self, val: Self::Value) -> Self {
        self.merge(Self::from_value(val))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct Sum<T>(pub T);

impl<T> From<T> for Sum<T> {
    #[inline]
    fn from(v: T) -> Sum<T> {
        Sum(v)
    }
}

impl<T> AsRef<T> for Sum<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Sum<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Aggregator for Sum<T>
where
    T: core::ops::Add<T, Output = T> + core::iter::Sum<T>,
{
    type Value = T;

    type Summary = T;

    #[inline]
    fn from_value(val: Self::Value) -> Self {
        Sum(val)
    }

    #[inline]
    fn summarize(self) -> Self::Summary {
        self.0
    }

    #[inline]
    fn empty() -> Self {
        Sum(core::iter::empty::<T>().sum())
    }

    #[inline]
    fn merge(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }

    #[inline]
    fn incorporate_before(self, val: Self::Value) -> Self {
        Sum(val + self.0)
    }

    #[inline]
    fn incorporate_after(self, val: Self::Value) -> Self {
        Sum(self.0 + val)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct Product<T>(pub T);

impl<T> From<T> for Product<T> {
    #[inline]
    fn from(v: T) -> Product<T> {
        Product(v)
    }
}

impl<T> AsRef<T> for Product<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Product<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Aggregator for Product<T>
where
    T: core::ops::Mul<T, Output = T> + core::iter::Product<T>,
{
    type Value = T;

    type Summary = T;

    #[inline]
    fn from_value(val: Self::Value) -> Self {
        Product(val)
    }

    #[inline]
    fn summarize(self) -> Self::Summary {
        self.0
    }

    #[inline]
    fn empty() -> Self {
        Product(core::iter::empty::<T>().product())
    }

    #[inline]
    fn merge(self, other: Self) -> Self {
        Product(self.0 * other.0)
    }

    #[inline]
    fn incorporate_before(self, val: Self::Value) -> Self {
        Product(val * self.0)
    }

    #[inline]
    fn incorporate_after(self, val: Self::Value) -> Self {
        Product(self.0 * val)
    }
}

#[repr(transparent)]
pub struct Count<T> {
    pub count: usize,
    phantom: core::marker::PhantomData<fn(T)>,
}

impl<T> Clone for Count<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

impl<T> Copy for Count<T> {}

impl<T> core::fmt::Debug for Count<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Count")
            .field("count", &self.count)
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl<T> core::hash::Hash for Count<T> {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.count.hash(state);
    }

    #[inline]
    fn hash_slice<H: core::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        // SAFETY: guaranteed by repr(transparent)
        let counts: &[usize] = unsafe { core::mem::transmute(data) };
        usize::hash_slice(counts, state)
    }
}

impl<T, U> PartialEq<Count<U>> for Count<T> {
    #[inline]
    fn eq(&self, other: &Count<U>) -> bool {
        self.count == other.count
    }

    #[inline]
    fn ne(&self, other: &Count<U>) -> bool {
        self.count != other.count
    }
}

impl<T> Eq for Count<T> {}

impl<T, U> PartialOrd<Count<U>> for Count<T> {
    #[inline]
    fn partial_cmp(&self, other: &Count<U>) -> Option<core::cmp::Ordering> {
        self.count.partial_cmp(&other.count)
    }

    #[inline]
    fn lt(&self, other: &Count<U>) -> bool {
        self.count < other.count
    }

    #[inline]
    fn le(&self, other: &Count<U>) -> bool {
        self.count <= other.count
    }

    #[inline]
    fn gt(&self, other: &Count<U>) -> bool {
        self.count > other.count
    }

    #[inline]
    fn ge(&self, other: &Count<U>) -> bool {
        self.count >= other.count
    }
}

impl<T> Ord for Count<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.count.cmp(&other.count)
    }

    #[inline]
    fn max(self, other: Self) -> Self {
        Self::from(self.count.max(other.count))
    }

    #[inline]
    fn min(self, other: Self) -> Self {
        Self::from(self.count.min(other.count))
    }

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self::from(self.count.clamp(min.count, max.count))
    }
}

impl<T> From<usize> for Count<T> {
    #[inline]
    fn from(v: usize) -> Count<T> {
        Count {
            count: v,
            phantom: core::marker::PhantomData,
        }
    }
}

impl<T> AsRef<usize> for Count<T> {
    #[inline]
    fn as_ref(&self) -> &usize {
        &self.count
    }
}

impl<T> AsMut<usize> for Count<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut usize {
        &mut self.count
    }
}

impl<T> From<Count<T>> for usize {
    #[inline]
    fn from(v: Count<T>) -> usize {
        v.count
    }
}

impl<T> Aggregator for Count<T> {
    type Value = T;

    type Summary = usize;

    #[inline]
    fn from_value(_: Self::Value) -> Self {
        Self::from(1)
    }

    #[inline]
    fn summarize(self) -> Self::Summary {
        self.count
    }

    #[inline]
    fn empty() -> Self {
        Self::from(0)
    }

    #[inline]
    fn merge(self, other: Self) -> Self {
        Self::from(self.count + other.count)
    }

    #[inline]
    fn incorporate_before(self, _: Self::Value) -> Self {
        Self::from(1 + self.count)
    }

    #[inline]
    fn incorporate_after(self, _: Self::Value) -> Self {
        Self::from(self.count + 1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct Min<T>(pub Option<T>);

impl<T> From<T> for Min<T> {
    #[inline]
    fn from(v: T) -> Min<T> {
        Min(Some(v))
    }
}

impl<T> Aggregator for Min<T>
where
    T: Ord,
{
    type Value = T;

    type Summary = Option<T>;

    #[inline]
    fn from_value(val: Self::Value) -> Self {
        Min(Some(val))
    }

    #[inline]
    fn summarize(self) -> Self::Summary {
        self.0
    }

    #[inline]
    fn empty() -> Self {
        Min(None)
    }

    #[inline]
    fn merge(self, other: Self) -> Self {
        match (self.0, other.0) {
            (Some(s), Some(o)) => Min(Some(s.min(o))),
            (None, o @ Some(_)) => Min(o),
            (s @ Some(_), None) => Min(s),
            (None, None) => Min(None),
        }
    }

    #[inline]
    fn incorporate_before(self, val: Self::Value) -> Self {
        if let Some(s) = self.0 {
            Min(Some(val.min(s)))
        } else {
            Min(Some(val))
        }
    }

    #[inline]
    fn incorporate_after(self, val: Self::Value) -> Self {
        if let Some(s) = self.0 {
            Min(Some(s.min(val)))
        } else {
            Min(Some(val))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct Max<T>(pub Option<T>);

impl<T> From<T> for Max<T> {
    #[inline]
    fn from(v: T) -> Max<T> {
        Max(Some(v))
    }
}

impl<T> Aggregator for Max<T>
where
    T: Ord,
{
    type Value = T;

    type Summary = Option<T>;

    #[inline]
    fn from_value(val: Self::Value) -> Self {
        Max(Some(val))
    }

    #[inline]
    fn summarize(self) -> Self::Summary {
        self.0
    }

    #[inline]
    fn empty() -> Self {
        Max(None)
    }

    #[inline]
    fn merge(self, other: Self) -> Self {
        match (self.0, other.0) {
            (Some(s), Some(o)) => Max(Some(s.max(o))),
            (None, o @ Some(_)) => Max(o),
            (s @ Some(_), None) => Max(s),
            (None, None) => Max(None),
        }
    }

    #[inline]
    fn incorporate_before(self, val: Self::Value) -> Self {
        if let Some(s) = self.0 {
            Max(Some(val.max(s)))
        } else {
            Max(Some(val))
        }
    }

    #[inline]
    fn incorporate_after(self, val: Self::Value) -> Self {
        if let Some(s) = self.0 {
            Max(Some(s.max(val)))
        } else {
            Max(Some(val))
        }
    }
}
