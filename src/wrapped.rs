use crate::DispersedIter;

pub struct WrappedIter<P, I> {
    pub(crate) part: P,
    pub(crate) inner: I,
}

impl<P, I> WrappedIter<P, I> {
    #[inline]
    pub fn unwrap(self) -> I {
        self.inner
    }
}

impl<'a, P, I> Iterator for WrappedIter<&'a P, I>
where
    I: DispersedIter<Part<'a> = &'a P>,
{
    type Item = I::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(self.part)
    }
}
