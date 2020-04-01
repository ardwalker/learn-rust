

#[cfg(test)]
mod tests {

    #[test]
    fn basic() {
        let a = 5;

        if a < 10 {
            assert_eq!(a, 5, "a < 10"); 
        }
        if a > 0 && a < 10 {
            assert_eq!(a, 5);
        }
    }

    #[test]
    fn assignment() {
        let answer = if true {
            "true"
        } else {
            "false"
        };
        assert_eq!(answer, "true");
    }
}



