use std::{collections::HashMap, marker::PhantomData};

use dispersed_iterators::DispersedIterator;

fn main() {
    let container = Container {
        indices: vec![1, 2],
        map: HashMap::from([(1, "hello".to_owned()), (2, "world".to_owned())]),
    };
    let mut nested_iter = TextByIndex(Indices(0, PhantomData), PhantomData); // we don't give any of them the container
    while let Some(value) = nested_iter.next(&container) {
        //                                             ^^^^ instead, we provide it here
        println!("{value}");
    }
}

struct Container {
    pub indices: Vec<i32>,
    pub map: HashMap<i32, String>,
}

struct Indices<'p>(usize, PhantomData<&'p ()>);

impl<'p> DispersedIterator for Indices<'p> {
    type Item = i32;
    type Part = &'p Container;

    fn next(&mut self, part: Self::Part) -> Option<Self::Item> {
        let index = part.indices.get(self.0);
        self.0 += 1;
        index.cloned()
    }
}

struct TextByIndex<'p, I: DispersedIterator<Item = i32, Part = &'p Container>>(
    I,
    PhantomData<&'p ()>,
);

impl<'p, I: DispersedIterator<Item = i32, Part = &'p Container>> DispersedIterator
    for TextByIndex<'p, I>
{
    type Item = &'p String;
    type Part = &'p Container;

    fn next(&mut self, part: Self::Part) -> Option<Self::Item> {
        self.0.next(part).and_then(|index| part.map.get(&index))
    }
}
