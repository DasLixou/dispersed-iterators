use wrapped::WrappedIter;

pub mod wrapped;

pub trait DispersedIter {
    type Item<'a>;
    type Part<'a>;

    fn next<'a>(&mut self, part: Self::Part<'a>) -> Option<Self::Item<'a>>;

    #[inline]
    fn into_wrapped(self, part: Self::Part<'_>) -> WrappedIter<Self::Part<'_>, Self>
    where
        Self: Sized,
    {
        WrappedIter { part, inner: self }
    }
}
