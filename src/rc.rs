

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::ops::Mul;

    #[test]
    fn simple_rc() {
        let five = Rc::new(5);
        assert_eq!(five.mul(5), 25);
    }

    #[test]
    fn rc_pin() {
        let _five = Rc::new("hello");
        // five.
        // assert_eq!(five.pin(), 25);

    }
}