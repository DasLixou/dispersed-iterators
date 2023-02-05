use dispersed_iterators::{wrapped::WrappedIter, DispersedIter};

fn main() {
    let vec = vec![1, 2, 3, 4];
    let mut multiplier = Multiplier(0, 3); // multiply value by 3
    while let Some(multiplied) = multiplier.next(&vec) {
        println!("{multiplied}");
    }
    // and now with all the utilities from `Iterator` by wrapping:
    let wrapped = WrappedIter {
        value: &vec,
        inner: Multiplier(0, 3),
    };
    let multiplied = wrapped.collect::<Vec<i32>>();
    for x in multiplied {
        println!("{x}");
    }
}

struct Multiplier(usize, i32);

impl DispersedIter for Multiplier {
    type Item<'a> = i32;
    type Part<'a> = &'a Vec<i32>;

    fn next<'a>(&mut self, part: Self::Part<'a>) -> Option<Self::Item<'a>> {
        let index = part.get(self.0);
        self.0 += 1;
        index.map(|i| *i * self.1)
    }
}
