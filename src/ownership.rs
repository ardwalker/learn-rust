

pub fn ownership() {
    println!("\n==============================================================");
    println!("Ownership");
    println!("==============================================================");

    // Use box to wrap and release values, like auto_ptr in C++

    // Calling Box::new(v) allocates some heap space, moves the value v into it,
    // and returns a Box pointing to the heap space.
    // Since a Box owns the space it points to, when the Box is dropped, it frees the space too.
    let point = Box::new((0.625, 0.5));  // point allocated here

    let label = format!("{:?}", point);  // label allocated here
    assert_eq!(label, "(0.625, 0.5)");
    let _pal = "palestrina".to_string();

    mover();
    copier();
    drop_when_reassigned();
    mutable_reassignment();
    moving_from_vector();
    move_vector();
    move_struct();
    // dropped here
}

fn mover() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    // let u = s;  -- ERROR USE OF MOVED VALUE

    println!(" t: {:?}", /* s, */ t);
    // ERROR                ^ value borrowed here after move
}

fn copier() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone();
    let u = s.clone();

    println!("s: {:?}, t: {:?}, u: {:?}", s, t, u);
}

fn drop_when_reassigned() {
    let mut _s = "Govinda".to_string();
    _s = "Siddhartha".to_string(); // value "Govinda" dropped here
}


// This time, t has taken ownership of the original string from s, so that by the
// time we assign to s, it is uninitialized. In this scenario, no string is dropped.
fn mutable_reassignment() {
    let mut _s = "Govinda".to_string();
    let _t = _s;
    _s = "Siddhartha".to_string(); // nothing is dropped here
}

fn moving_from_vector() {
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
}

fn move_vector() {
    let v = vec![
                    "liberté".to_string(),
                    "égalité".to_string(),
                    "fraternité".to_string()];

    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
    // let _a = v.capacity();
    // ERROR
    //       ^ value borrowed here after move
}



// Since Label is not Copy, passing it to print moved ownership of the value to
// the print function, which then dropped it before returning.
fn move_struct() {
    struct Label { number: u32 };
    fn print(l: Label) { println!("STAMP: {}", l.number); }
    let l = Label { number: 3 };
    print(l);
    // println!("My label number is: {}", l.number);
    /*
    |     let l = Label { number: 3 };
    |         - move occurs because `l` has type `ownership::move_struct::Label`, which does not implement the `Copy` trait
    |     print(l);
    |           - value moved here
    |     println!("My label number is: {}", l.number)
    |                                        ^^^^^^^^ value borrowed here after move
    */
}