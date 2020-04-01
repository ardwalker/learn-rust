

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

}