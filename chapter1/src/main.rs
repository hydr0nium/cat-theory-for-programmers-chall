fn main() {}

#[allow(dead_code)]
fn id<T>(x: T) -> T {
    x
}

#[allow(dead_code)]
fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x| g(f(x))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_id() {
        assert_eq!("Hello World", id("Hello World"));
    }

    #[test]
    fn test_compose() {
        let f = |a: i32| return a + 1;
        let g = |b: i32| return b * b;
        assert_eq!(36, compose(f, g)(5));
    }

    #[test]
    fn test_compose_id() {
        let f = |a: i32| return a + 1;
        for n in 0..=15 {
            assert_eq!(f(n), compose(f, id)(n));
            assert_eq!(f(n), compose(id, f)(n));
        }
    }
}
