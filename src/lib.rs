pub trait DispersedIterator {
    type Item<'a>;
    type Part<'a>;

    fn next<'a: 'b, 'b>(&mut self, part: Self::Part<'a>) -> Option<Self::Item<'b>>;
}

pub trait NestedDispersedIterator {
    type Item<'a>;
    type Part<'a>;
    type Inner<'a>;

    fn next<'a: 'b, 'b>(
        &mut self,
        part: Self::Part<'a>,
        inner: Self::Inner<'a>,
    ) -> Option<Self::Item<'b>>;
}

impl<T: NestedDispersedIterator> DispersedIterator for T {
    type Item<'a> = <T as NestedDispersedIterator>::Item<'a>;
    type Part<'a> = (
        <T as NestedDispersedIterator>::Part<'a>,
        <T as NestedDispersedIterator>::Inner<'a>,
    );

    fn next<'a: 'b, 'b>(&mut self, part: Self::Part<'a>) -> Option<Self::Item<'b>> {
        <T as NestedDispersedIterator>::next(self, part.0, part.1)
    }
}
