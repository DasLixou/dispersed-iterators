use std::collections::HashMap;

use dispersed_iterators::DispersedIter;

fn main() {
    let mut container = Container {
        indices: vec![1, 2],
        map: HashMap::from([(1, "hello".to_owned()), (2, "world".to_owned())]),
    };
    let mut nested_iter = TextByIndexMut(Indices(0)); // we don't give any of them the container
    while let Some(value) = nested_iter.next(&mut container) {
        println!("{value}");
    }
}

struct Container {
    pub indices: Vec<i32>,
    pub map: HashMap<i32, String>,
}

struct Indices(usize);

impl DispersedIter for Indices {
    type Item<'a> = &'a i32;
    type Part<'a> = &'a Container;

    fn next<'a>(&mut self, part: Self::Part<'a>) -> Option<Self::Item<'a>> {
        let index = part.indices.get(self.0);
        self.0 += 1;
        index
    }
}

struct TextByIndexMut<I>(I);

impl<I> DispersedIter for TextByIndexMut<I>
where
    I: for<'a> DispersedIter<Item<'a> = &'a i32, Part<'a> = &'a Container>,
{
    type Item<'a> = &'a String;
    type Part<'a> = &'a mut Container;

    fn next<'a>(&mut self, part: Self::Part<'a>) -> Option<Self::Item<'a>> {
        self.0.next(part).and_then(|index| part.map.get(index))
    }
}
