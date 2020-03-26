
pub fn vectors() {

    let f: Vec<String> = std::env::args().skip(1).collect();

    for i in f {
        i.into_boxed_str();
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn vector_fold() {
        let a = vec![2, 3, 5, 7];
        assert_eq!(a.iter().fold(1, |a, b| a * b), 210);
    }

    #[test]
    fn vector_push() {
        let mut a = vec![2, 3, 5, 7];
        a.push(11);
        assert_eq!(a, vec![2,3,5,7,11]);
    }

    #[test]
    fn vector_new() {
        let mut c = Vec::new(); // vec! is short for Vec::new()
        c.push("valid");
        c.push("example");
        assert_eq!(c, vec!["valid","example"]);
    }

    #[test]
    fn vector_new_fill() {
        let b = vec![5; 2 * 6]; // Creates array of 12 filling with 5
        assert_eq!(b, vec![5,5,5,5,5,5,5,5,5,5,5,5]);
    }

    #[test]
    fn vector_range() {
        let d: Vec<i32>  = (0..7).collect();
        assert_eq!(d, [0,1,2,3,4,5,6]);
    }

    #[test]
    fn vector_capacity() {
        let e: Vec<String> = Vec::with_capacity(10);
        assert_eq!(e.len(), 0);
        assert_eq!(e.capacity(), 10);
    }

}