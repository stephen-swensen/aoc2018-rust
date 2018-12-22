use std::collections::HashMap;
use std::hash::Hash;

pub trait IteratorExt<T>: Iterator<Item=T> + Sized {
    ///Group elements into a HashMap by the key generating function.
    fn group_by_key<U: Eq + Hash>(self, f:fn(&T) -> U) -> HashMap<U,Vec<T>> {
        let mut grouping: HashMap<U,Vec<T>> = HashMap::new();
        for item in self {
            let key = f(&item);
            match grouping.get_mut(&key) {
                Some(value) => value.push(item),
                None => {
                    let value = vec![item];
                    grouping.insert(key, value);
                    ()
                }
            }
        }
        grouping
    }

    fn sum_by_key(self, f:fn(&T) -> i32) -> i32 {
        self.map(|x| f(&x)).sum()
    }
}

impl<T,U> IteratorExt<T> for U where U: Iterator<Item=T> { }