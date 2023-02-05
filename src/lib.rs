pub trait DispersedIterator {
    type Item;
    type Part;

    fn next(&mut self, part: Self::Part) -> Option<Self::Item>;
}
