

#[cfg(test)]
mod test {

    #[test]
    fn references1() {
        let mut numbers = [3, 1, 2];
        numbers.sort(); // implicitly borrows a reference 
        (&mut numbers).sort();
    }

    #[test]
    fn references2() {
        fn calculate_length(s: &String) -> usize { s.len() }
        let s1 = String::from("hello");
        assert_eq!(calculate_length(&s1), 5); // takes a reference so no copy needed
    }

    #[test]
    fn references3() {
        let x = 10;
        let r = &x;         // &x is a shared reference to x
        assert_eq!(*r, 10); // explicitly dereference r
    }    

    #[test]
    fn explicit_deref() {
        struct Anime { name: &'static str, _bechdel_pass: bool };
        let aria = Anime { name: "Aria: The Animation", _bechdel_pass: true };
        let anime_ref = &aria;
        assert_eq!((*anime_ref).name, "Aria: The Animation");
    }

    // Since references are so widely used in Rust, the . operator implicitly dereferences its
    // left operand, if needed
    #[test]
    fn implicit_deref() {
        struct Anime { name: String, _bechdel_pass: bool };
        let aria = Anime { name: "Aria".to_string(), _bechdel_pass: true };
        let aref = &aria;
        assert_eq!(aref.name, "Aria"); // Implicit dereference
        assert_eq!((*aref).name, "Aria"); // explicit dereference
    }

    #[test]
    fn double_deref() {
        fn is_ten(val: &i32) -> bool { 
            *val == 10  // note no double deref required
        }
        let x = 10;
        let r = &x;
        let rr = &r;                    // `rr` is a `&&x`
        assert_eq!(**rr, 10);           // double deref ** 
        assert_eq!(*rr, &10);           // double deref ** 
        assert!(**rr == 10);           // double deref ** 
        assert_eq!(is_ten(rr), true);   // comparison operator traverse reference chain
    }


    #[test]
    fn assigning_references() {
        let x = 10;
        let y = 20;

        let mut _r = &x;
        _r = &y; // r now points to y

        assert_eq!(*_r, 20);
    }


    // The . operator can also implicitly borrow a reference to its left operand,
    // if needed for a method call. For example, Vec’s sort method takes a mutable
    // reference to the vector, so the two calls shown here are equivalent
    #[test]
    fn mutable_ref() {
        let mut v = vec![1973, 1968];
        v.sort();  // implicit ref ; 
        // (&mut v).sort() is explicit version, which is a not as nice
        assert_eq!(v , vec![1968,1973] );
    }



    // == operator follows all the references and performs the comparison on their
    // final targets, x and y.
    #[test]
    fn comparing_references() {
        let x = 10;
        let y = 10;

        let rx = &x;
        let ry = &y;

        let rrx = &rx; // reference to a reference
        let rry = &ry;

        assert!(rrx == rry);
    }

    #[test]
    fn reference_and_address_equality() {
        let x = 10;
        let y = 10;

        let rx = &x;
        let ry = &y;

        assert!(rx == ry);                        // their referents are equal
        assert_eq!(std::ptr::eq(rx, ry), false);  // but occupy different addresses
    }

    #[test]
    fn arbitrary_expressions() {
        fn factorial(n: usize) -> usize {
            (1..n+1).fold(1, |a, b| a * b)
        }
        let r = &factorial(6);
        assert_eq!(r + &1009, 1729);
    }

}

