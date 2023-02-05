pub trait DispersedIterator {
    type Item<'a>;
    type Part<'a>;

    fn next<'a: 'b, 'b>(&mut self, part: Self::Part<'a>) -> Option<Self::Item<'b>>;
}
