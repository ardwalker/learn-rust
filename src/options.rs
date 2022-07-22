

#[cfg(test)]
mod tests {
    #[test]
    fn is_some_for_some() {
        let x: Option<u32> = Some(2);
        assert_eq!(x.is_some(), true);                
    }


    #[test]
    fn is_some_for_none() {
        let x: Option<u32> = None;
        assert_eq!(x.is_some(), false);        
    }

    #[test]
    #[ignore]
    fn name() {
        let x: i8 = 5;
        let y: Option<i8> = Some(5);


        // let sum = Some(x: i8) + y;
        assert_eq!(x,5);
        assert_eq!(y,Some(5));
        // assert_eq!(sum, 10);
    }
}