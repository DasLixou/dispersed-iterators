use std::collections::HashMap;

use dispersed_iterators::{DispersedIterator, NestedDispersedIterator};

fn main() {
    let mut container = Container {
        indices: vec![1, 2],
        map: HashMap::from([(1, "hello".to_owned()), (2, "world".to_owned())]),
    };
    let mut indices = Indices(0);
    let mut nested_iter = TextByIndexMut; // we don't give any of them the container
    while let Some(value) =
        DispersedIterator::next(&mut nested_iter, (&mut container, &mut indices))
    {
        println!("{value}");
    }
}

struct Container {
    pub indices: Vec<i32>,
    pub map: HashMap<i32, String>,
}

struct Indices(usize);

impl DispersedIterator for Indices {
    type Item<'a> = &'a i32;
    type Part<'a> = &'a Container;

    fn next<'a: 'b, 'b>(&mut self, part: Self::Part<'a>) -> Option<Self::Item<'b>> {
        let index = part.indices.get(self.0);
        self.0 += 1;
        index
    }
}

struct TextByIndexMut;

impl NestedDispersedIterator for TextByIndexMut {
    type Item<'a> = &'a String;
    type Inner<'a> = &'a mut Indices; // TODO: Don't specify it here directly
    type Part<'a> = &'a mut Container;

    fn next<'a: 'b, 'b>(
        &mut self,
        part: Self::Part<'a>,
        inner: Self::Inner<'a>,
    ) -> Option<Self::Item<'b>> {
        inner.next(part).and_then(|index| part.map.get(index))
    }
}
