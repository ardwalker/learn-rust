


#[cfg(test)] 
mod tests {

    #[test]
    fn name() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
            if x.len() > y.len() {
                x
            }
            else {
                y
            }
        }
        let s1 = "test1".to_string();
        let s2 = "test22".to_string();

        assert_eq!(longest(&s1, &s2), "test22");

    }
}
