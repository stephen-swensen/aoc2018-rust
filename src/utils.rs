use std::collections::HashMap;
use std::hash::Hash;


//http://xion.io/post/code/rust-extension-traits.html
//https://stackoverflow.com/questions/33376486/is-there-a-way-other-than-traits-to-add-methods-to-a-type-i-dont-own
//https://doc.rust-lang.org/stable/std/?search=trait%3AExt
///Extention trait for Vec<T>
pub trait VecExt<T> {
    fn group_by_key<U: Eq + Hash>(self, f:fn(&T) -> U) -> HashMap<U,Vec<T>>;
}

impl<T> VecExt<T> for Vec<T> { 
    ///Group elements into a HashMap by the key generating function.
    fn group_by_key<U: Eq + Hash>(self: Vec<T>, f:fn(&T) -> U) -> HashMap<U,Vec<T>> {
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
}

pub trait IteratorExt<T>: Iterator<Item=T> + Sized {
    fn sum_by_key(self, f:fn(&T) -> i32) -> i32 {
        self.map(|x| f(&x)).sum()
    }
}

impl<T,U> IteratorExt<T> for U where U: Iterator<Item=T> { }