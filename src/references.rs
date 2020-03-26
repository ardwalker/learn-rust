
use std::collections::HashMap;

// Non-owning pointer types: references
// which have no effect on their referents’ lifetimes

// A shared reference lets you read but not modify its referent.
// However, you can have as many shared references to a particular value at a time as you like.
// The expression &e yields a shared reference to e’s value;
// if e has the type T, then &e has the type &T, pronounced “ref T”. Shared references are Copy.

// If you have a mutable reference to a value, you may both read and modify the value.
// However, you may not have any other references of any sort to that value active at the same time.
// The expression &mut e yields a mutable reference to e’s value; you write its type as &mut T
// which is pronounced “ref mute T”. Mutable references are not Copy.

type Table = HashMap<String, Vec<String>>;


pub fn references() {
    println!("\n==============================================================");
    println!("References");
    println!("==============================================================");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    borrow_mutable_reference();
    assigning_references();
    comparing_references();
    reference_and_address_equality();
    arbitrary_expressions();
    reference_scope();
    table();
}


// The . operator can also implicitly borrow a reference to its left operand,
// if needed for a method call. For example, Vec’s sort method takes a mutable
// reference to the vector, so the two calls shown here are equivalent
fn borrow_mutable_reference() {
    let mut v = vec![1973, 1968];
    v.sort();           // implicitly borrows a mutable reference to v
    (&mut v).sort();    // equivalent; much uglier
}


fn assigning_references() {
    let x = 10;
    let y = 20;
    let mut r = &x;

    let b = true;

    if b { r = &y; } // r now points to y

    assert!(*r == 10 || *r == 20);
}

// == operator follows all the references and performs the comparison on their
// final targets, x and y.
fn comparing_references() {
    let x = 10;
    let y = 10;

    // reference
    let rx = &x;
    let ry = &y;

    // reference to a reference
    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);
}


fn reference_and_address_equality() {
    let x = 10;
    let y = 10;

    // reference
    let rx = &x;
    let ry = &y;

    assert!(rx == ry);                      // their referents are equal
    assert!(!std::ptr::eq(rx, ry));   // but occupy different addresses
}


// Whereas C and C++ only let you apply the & operator to certain kinds of expressions,
// Rust lets you borrow a reference to the value of any sort of expression at all:
fn arbitrary_expressions() {
    fn factorial(n: usize) -> usize {
        (1..n+1).fold(1, |a, b| a * b)
    }
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
}

// Rust’s complaint is that x lives only until the end of the inner block,
// whereas the reference remains alive until the end of the outer block,
// making it a dangling pointer, which is verboten.
fn reference_scope() {
    let _r;
    {
        let x = 1;
        _r = &x;
    }
    //assert_eq!(*_r, 1);  // bad: reads memory `x` used to occupy
    //         ^^ borrowed value does not live long enough
}


fn calculate_length(s: &String) -> usize {
    s.len()
}

fn table() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                      "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(),
                      "a salt cellar".to_string()]);

    sort_works(&mut table);
    show(&table);
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}


#[cfg(test)]
mod test {

    #[test]
    fn simple_shared_ref() {
        let x = 10;
        let r = &x;        // &x is a shared reference to x
        assert_eq!(*r, 10);      // explicitly dereference r
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
        struct Anime { name: &'static str, _bechdel_pass: bool };
        let aria = Anime { name: "Aria", _bechdel_pass: true };
        let anime_ref = &aria;
        assert_eq!(anime_ref.name, "Aria"); // Implicit dereference
    }

    #[test]
    fn mutable_ref() {
        let mut v = vec![1973, 1968];
        v.sort();  // implicit ref ; (&mut v).sort() is explicit version
        assert_eq!(v , vec![1968,1973] );
    }

}

