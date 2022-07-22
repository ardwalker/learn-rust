// enum Result<T, E> {
//    Ok(T),
//    Err(E),
// }


#[cfg(test)]
mod tests {

    fn do_a() -> Result<i32,String> {
        Ok(1)
    }
    
    fn do_b(a: &i32) -> Result<i32,String> {
        Ok(a + 1)
    }

    #[test]
    fn test1() {
        let s: Result<i32,String> = match do_a() {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        };

        assert_eq!(s.is_ok(), true);
        assert_eq!(*s.as_ref().unwrap(), 1);

        if s.is_ok() {
            let t: Result<i32,String> = match do_b(s.as_ref().unwrap()) {
                Ok(v) => Ok(v),
                Err(e) => Err(e.to_string()),
            };
            assert_eq!(t.is_ok(), true);
            assert_eq!(t.unwrap(), 2);
        }
    }

    pub fn do_a_and_b() -> Result<i32,String> {
        Ok(do_b(&do_a()?)?)
    }

    #[test]
    fn test2() {
        let x = do_a_and_b();
        assert_eq!(x.unwrap(), 2);
    }

}