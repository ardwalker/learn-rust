
struct Point {x: i32, y: i32}

#[cfg(test)]
mod tests {

    #[test]
    fn literal_match() {
        let a: i32 = 0;
        let b = match a {
                0 => "zero",
                1 => "one",
                _ => "unknown"
            };
        assert_eq!("zero", b);
    }

    #[test]
    #[ignore]
    fn range() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn wildcard() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn variable() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn ref_var() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn binding() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn enums() {
        unimplemented!();
    }

    #[test]
    fn tuple_patterns() {
        let (x, y) = (1,3);
        let a = match(x,y) {
            (0, 0) => "zeros",
            (_, 0) => "right-zero",
            (0, _) => "left-zero",
            (_, _) => "anything"
        };
        assert_eq!(a, "anything");
    }

    #[test]
    fn structs() {
        use super::Point;
        let pt = Point { x: 0, y: 1 };
        
        let a  = match pt {
            Point { x: length, y: 0 } => length.to_string(),
            Point { x: 0, y: 1 } => "y-is-one".to_string(),
            Point { x: 5, y: 1 } => "x-is-five".to_string(),
            Point { x: _, y: _ } => "anything".to_string()
        };

        assert_eq!(a, "y-is-one");

    }

    #[test]
    #[ignore]
    fn reference() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn multiple() {
        unimplemented!();
    }

    #[test]
    #[ignore]
    fn guard_expr() {
        unimplemented!();
    }

}
