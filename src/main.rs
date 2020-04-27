
// https://doc.rust-lang.org/std/index.html#keywords

mod arrays;
mod references;
mod vectors;
mod slicing;
mod tuple;
mod strings;
mod ownership;
mod iterator;
mod oracle_usage;
mod closures;
mod modules;
mod structs;
mod implementation;
mod generics;
mod rc;
mod pattern;
mod conversions;
mod conditionals;
mod loops;
mod lifetimes;
mod mutability;


fn main() {
    println!("Hello, world!");

    let a = [1,2,3];
    println!("{:?}", a);

    // Tuples
    let t1 = (1, 1.5, true, 'a');
    println!("{:?}", t1);

    let mut t2 = (1, 1.5);
    t2.0 = 2;
    t2.1 = 3.0;
    println!("t2: {:#?}", t2);


    pattern_matching();
    vectors::vectors();
    slicing::slicing();
    strings::strings();
    ownership::ownership();
    iterator::iterator();
    // oracle_usage::oracle_usage();
}

fn pattern_matching() {

    println!("-> Pattern matching tuples");
    let t1 = (1, 1.5, true, 'a');
    let t2 = (2, 3.0);

    let (c, d) = t2;        // c = 2, d = 3.0
    let (e, _, _, f) = t1; // e = 1, f = 'a'
    let g = (0,);               // single-element tuple
    let h = (t2, (2, 4), 5);     // ((2, 3.0), (2, 4), 5)
    println!("c: {:?}, d: {:?}, e: {:?}, f: {:?}, g: {:?}, h: {:?}", c, d, e, f, g, h);

    let tshirt_width = 20;
    let tshirt_size = match tshirt_width {
        16        => "S",   // check 16
        17 | 18   => "M",   // check 17 and 18
        19 ..= 21 => "L",   // check from 19 to 21 (19,20,21)
        22        => "XL",
        _         => "Not Available",
    };
    assert_eq!(tshirt_size, "L");
}



