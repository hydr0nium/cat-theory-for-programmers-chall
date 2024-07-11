use std::{collections::HashMap, hash::Hash};

struct Memorizer<A, B> {
    cache: HashMap<A, B>,
}

fn main() {}

impl<'a, A: 'a, B: 'a> Memorizer<A, B> {
    pub fn memorize(&'a mut self, f: impl Fn(A) -> B + 'a) -> impl FnMut(A) -> B + 'a
    where
        A: Eq + Hash + Clone,
        B: Clone,
    {
        move |x: A| {
            if self.cache.contains_key(&x) {
                return self.cache.get(&x).unwrap().clone();
            }
            let res = f(x.clone());
            self.cache.insert(x, res.clone());
            return res;
        }
    }

    fn new() -> Memorizer<A, B> {
        Memorizer {
            cache: HashMap::new(),
        }
    }
}

mod tests {

    use std::{thread, time};

    use super::*;

    #[test]
    fn test_memorize() {
        fn addone(n: i32) -> i32 {
            let ten_millis = time::Duration::from_millis(5000);
            thread::sleep(ten_millis);
            n + 1
        }
        let mut m: Memorizer<i32, i32> = Memorizer::new();
        let mut memaddone = m.memorize(addone);
        assert_eq!(2, memaddone(1));
        assert_eq!(2, memaddone(1));
        assert_eq!(4, memaddone(3));
        assert_eq!(4, memaddone(3));
        assert_eq!(2, memaddone(1));
        //assert_eq!(3, memaddone(2));
    }
}
