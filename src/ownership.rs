

/* Ownership 
    From: https://blog.thoughtram.io/ownership-in-rust/

    Every value has a single owner that determines its lifetime.

    let s = "Have a nice day".to_string();

    When the owner of some value is freed or dropped, the owned value is dropped as well. 
    When are values dropped? This is where it gets interesting. 
    When the program leaves a block in which a variable is declared, that variable will be dropped, dropping its value with it.
    
    A block could be a function, an if statement, or pretty much anything that introduces a new code block with curly braces
    
    
  Moves and Borrowing
    Rust ensures that a single variable owns a value
    Rust moves values to a new owner : 
        when values are assignmed 
        when passing values to functions

*/

/**

Each value in Rust has a variable that’s called its owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
**/

// borrow-operator &

pub fn ownership() {

    // Use box to wrap and release values, like auto_ptr in C++

    // Calling Box::new(v) allocates some heap space, moves the value v into it,
    // and returns a Box pointing to the heap space.
    // Since a Box owns the space it points to, when the Box is dropped, it frees the space too.
    let point = Box::new((0.625, 0.5));  // point allocated here

    let label = format!("{:?}", point);  // label allocated here
    assert_eq!(label, "(0.625, 0.5)");
    let _pal = "palestrina".to_string();

    // dropped here
}





#[cfg(test)]
mod tests {

    #[test]
    fn ownership1() {

        // This function doesn’t need ownership to output the value it takes. 
        // Also, it would prevent us from calling the function multiple times with the same variable:

        let name = "Pascal".to_string();
        greet(name); // Moves string 'name' to the greet function

        fn greet(name: String) {
            assert_eq!(name.len(), 6); 
            // greet scope ends, 'name' is dropped
        }

    }


    #[test]
    fn ownership2() {
        // To get a reference to a variable we use the & symbol. 
        // We can be explict about when we expect a reference over a value
        let name = "Pascal".to_string();
        greet(&name); // borrows name
        greet(&name); // works again

        fn greet(name: &String) {
            assert_eq!(name.len(), 6); 
        }
        // name never loses ownership of its value 
        // and a and b are just pointers to the same data. 
    }

    #[test]
    fn ownership3() {
        // When a function expects a reference to a value, it *borrows it. 
        // Notice that it never gets ownership of the values that are being passed to it.
        // We can address the variable assignment from earlier in a similar fashion:
        let name = "Pascal".to_string();
        let a = &name;  // borrow ref to name
        let b = &name;  // borrow ref to name
        let c = b;      // c is again a referece

        greet(&name);   // borrows name
        greet(&a);      // works again
        greet(&b);      // works again
        greet(&c);      // works again

        fn greet(name: &String) {
            assert_eq!(name.len(), 6); 
        }
    }

    #[test]
    fn ownership4() {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = s;  // Ownership is moved to 't'
        // let u = s;  -- ERROR USE OF MOVED VALUE

        println!(" t: {:?}", /* s, */ t);
        // ERROR                ^ value borrowed here after move
    }

    #[test]
    fn ownership5() {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = s.clone();
        let u = s.clone();

        assert_eq!(s.len(), 3);
        assert_eq!(t.len(), 3);
        assert_eq!(u.len(), 3);
    }

    #[test]
    fn ownership6() {
        // When a mutable reference is assigned a new value, the 
        // old value is dropped
        let mut _s = "Govinda".to_string();
        _s = "Siddhartha".to_string(); // value "Govinda" dropped here
        assert_eq!(_s, "Siddhartha");
    }


    // This time, t has taken ownership of the original string from s, so that by the
    // time we assign to s, it is uninitialized. In this scenario, no string is dropped.
    #[test]
    fn ownership7() {
        let mut _s = "Govinda".to_string();
        let _t = _s;
        _s = "Siddhartha".to_string(); // nothing is dropped here
        assert_eq!(_s, "Siddhartha");
        assert_eq!(_t, "Govinda");
    }

    #[test]
    fn ownership8() {
        let mut v = Vec::new();
        for i in 101 .. 106 {
            v.push(i.to_string());
        }

        // ERROR CANNOT MOVE...
        // let third = v[2];
        // let fifth = v[4];
        // ERROR CANNOT MOVE
        //             ^^^^
        //              help: consider using a reference instead `&v[2]`”
        let _forth = &v[2];
        assert_eq!(_forth,"103");
    }

    #[test]
    fn ownership9() {
        let v = vec!["liberté".to_string(), "égalité".to_string(),
                        "fraternité".to_string()];
        
        // Loop block moves moves owenership of v
        for mut s in v {
            s.push('!');
             println!("{}", s);
            assert!(s.len() > 0 );
        }
        // let _a = v.capacity();
        // ERROR
        //       ^ value borrowed here after move
    }

}