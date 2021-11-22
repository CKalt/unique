use std::collections::HashSet;
use std::hash::Hash;

struct Unique<I>
where
    I: Iterator,
{
    seen: HashSet<I::Item>,
    underlying: I,
}

impl<I> Iterator for Unique<I>
where
    I: Iterator,
    I::Item: Hash + Eq + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.underlying.next() {
            if !self.seen.contains(&x) {
                self.seen.insert(x.clone());
                return Some(x);
            }
        }
        None
    }
}

trait UniqueExt: Iterator {
    fn unique(self) -> Unique<Self>
    where
        Self::Item: Hash + Eq + Clone,
        Self: Sized,
    {
        Unique {
            seen: HashSet::new(),
            underlying: self,
        }
    }
}

impl<I: Iterator> UniqueExt for I {}

fn main() {
    let foo = vec!["cc", "a", "z", "b", "a", "cc", "cc", "d","d","z"];

    for s in foo.iter().unique() {
        println!("{}", s);
    }
}
