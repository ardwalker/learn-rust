
// use std::iter::Iterator;


pub fn iterator() {
}


#[cfg(test)]
mod tests {
    use std::ops::Mul;
    use std::cmp::Ordering;

    #[test]
    fn all() {
        let a = [1, 2, 3];
        assert!(a.iter().all(|&x| x > 0));
    }

    #[test]
    fn any() {
        let a = [1, 2, 3];
        assert_eq!(a.iter().any(|&x| x > 0), true);
    }

    // collect() can take anything iterable, and turn it into a relevant collection.
    // This is one of the more powerful methods in the standard library,
    // used in a variety of contexts
    #[test]
    fn collect() {
        let a = vec![1, 2, 3];
        let b: Vec<_> = a.iter().map(|x| *x).collect();
        // .map(|&x| x * 2)
        // .collect();

        assert_eq!(vec![1, 2, 3], b);
    }

    #[test]
    fn map() {
        let a = vec![1, 2, 3];
        let b: Vec<i32> = a.iter().map(|&n| n.mul(2)).collect();
        assert_eq!(b, [2, 4, 6]);
    }

    #[test]
    fn map2() {
        let a = [1, 2, 3];

        let mut iter = a.iter();
        let sum: i32 = iter.by_ref().take(2).fold(0, |acc, i| acc + *i);
        assert_eq!(sum, 3);
        // now this is just fine:
        assert_eq!(iter.next(), Some(&3));
    }

    #[test]
    fn chain1() {
        // Takes two iterators and creates a new iterator over both in sequence.
        let a1 = [1, 2, 3];
        let a2 = [4, 5, 6];

        let mut iter = a1.iter().chain(a2.iter());
        let _a3: Vec<_> = iter.by_ref().collect();

        assert_eq!(_a3, vec![&1, &2, &3, &4, &5, &6]);
    }

    #[test]
    fn chain2() {

        // Takes two iterators and creates a new iterator over both in sequence.
        let a1 = [1, 2, 3];
        let a2 = [4, 5, 6];

        let mut iter = a1.iter().chain(a2.iter());

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn cloned() {
        let a = [1, 2, 3];
        let v_cloned: Vec<_> = a.iter().cloned().collect();

        // cloned is the same as .map(|&x| x), for integers
        let v_map: Vec<_> = a.iter().map(|&x| x).collect();

        assert_eq!(v_cloned, vec![1, 2, 3]);
        assert_eq!(v_map, vec![1, 2, 3]);
    }

    #[test]
    fn cmp() {
        assert_eq!([1].iter().cmp([1].iter()), Ordering::Equal);
        assert_eq!([1].iter().cmp([1, 2].iter()), Ordering::Less);
        assert_eq!([1, 2].iter().cmp([1].iter()), Ordering::Greater);
    }

    #[test]
    fn count() {
        let a = [1, 2, 3, 4, 5];
        assert_eq!(a.len(), 5);
        assert_eq!(a.iter().count(), 5);
    }

    #[test]
    fn cycle() {
        let a = [1, 2, 3];

        // Will cycle forever
        let mut it = a.iter().cycle();

        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&1));
    }

    #[test]
    fn enumerate() {
        let a = ["will", "this", "work"];
        let iter = a.iter().enumerate();
        let v: Vec<_> = iter.collect();
        assert_eq!(v, vec![(0, &"will"),(1, &"this"),(2, &"work")]);
    }

    #[test]
    fn eq() {
        assert_eq!([1].iter().eq([1].iter()), true);
        assert_eq!([1].iter().eq([1, 2].iter()), false);
    }
}
