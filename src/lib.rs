pub trait DispersedIter {
    type Item<'a>;
    type Part<'a>;

    fn next<'a>(&mut self, part: Self::Part<'a>) -> Option<Self::Item<'a>>;
}
