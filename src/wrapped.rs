use crate::DispersedIter;

pub struct WrappedIter<T, I> {
    pub value: T,
    pub inner: I,
}

impl<'a, T, I> Iterator for WrappedIter<&'a T, I>
where
    I: DispersedIter<Part<'a> = &'a T>,
{
    type Item = I::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(self.value)
    }
}
