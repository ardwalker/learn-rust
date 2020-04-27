

#[allow(dead_code)]
pub fn closures() {
    cl1();
}

fn cl1() {
    let is_even = |x| x % 2 == 0;
    is_even(3);
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_simple_closure() {
        let is_even = |x| x % 2 == 0;
        assert_eq!(is_even(3), false);
    }

    #[test]
    fn test_simple_closure_written_out() {
        let is_even = |x:u64| -> bool { x % 3 == 0 };
        assert_eq!(is_even(3), true); 
    }

    fn call_twice<F>(closure: F) where F: FnOnce() {
        closure();
        // closure();
    }

    #[test]
    fn drop_closure() {
        let my_string = "hello".to_string();
        let f = || drop(my_string);

        call_twice(f);
        // assert f implements FnOnce
    }

    #[test]
    fn mutable_closure() {
        let mut i = 0;
        let mut myfn = || -> i32 { i += 1; i };
        let x = myfn();
        assert_eq!(i, 1);
        assert_eq!(x, 1);
    }
}