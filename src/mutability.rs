

#[cfg(test)]
mod tests {
    #[test]
    fn simple_mutable() {
        // This is a mutable variable binding. When a binding is mutable, 
        // it means you’re allowed to change what the binding points to. 
        // So in the above example, it’s not so much that the value at x 
        // is changing, but that the binding changed from one i32 to another.
        #[allow(unused_assignments)]
        let mut x = 5;
        x = 6;
        assert_eq!(x, 6);
    }

    #[test]
    fn reference_to_mutable() {
        /* 
         * You can also create a reference to it, using &x, but if you want to 
         * use the reference to change it, you will need a mutable reference:
         */
        let mut x = 5;

        /*
         *  y is an immutable binding to a mutable reference, which means that you 
         * can’t bind 'y' to something else (y = &mut z), but y can be used to 
         * bind x to something else (*y = 5). A subtle distinction.
         */
        let y = &mut x;
        *y += 1;
        assert_eq!(*y, 6);
    }

    #[test]
    fn mutable_binding_to_mutable_reference() {
        let mut x = 5;

        let mut y = &mut x; // mutable binding to mutable reference

        *y = 6;             // change value of x, because of &mut x
        assert_eq!(*y, 6);
        assert_eq!(x, 6);

        let mut z = 7;
        y = &mut z;         // point to a new reference, because of let mut y
        assert_eq!(*y, 7);
        assert_eq!(z, 7);


    }

    #[test]
    fn destructuring_to_mutable() {
        let (mut x, y) = (5, 6);
        x += 2;
        assert_eq!(y, 6);
        assert_eq!(x, 7);
    }

    #[test]
    fn mutable_single_line() {
        let mut list = vec![1, 2, 3];
        *list.first_mut().expect("list was empty") += 1;

        let list_first = list.first();
        let list_last  = list.last();

        assert_eq!(*list_first.unwrap(), 2);
        assert_eq!(*list_last.unwrap(), 3);
    }

    #[derive(PartialEq,Debug)]
    struct Point1 {
        x: i32,
        y: i32, // cannot make a field mutable
    }

    #[test]
    fn field_level_struct_mutability() {
        let mut a = Point1 { x: 1, y: 2 };
        a.x = 10;  // can do this because of let mut a
        a.y = 20;
        assert_eq!(a, Point1 {x: 10, y: 20 });
    }

    use std::cell::Cell;

    #[derive(PartialEq,Debug)]
    struct Point2 {
        x: i32,
        y: Cell<i32>,
    }

    #[test]
    fn field_struct_mutability_cell() {
        let a = Point2 { x: 1, y: Cell::new(2) };
        //a.x = 10;  // can do this because 'a' binding is not mutable
        a.y.set(20); // can change value of y without mutable binding...
        assert_eq!(a, Point2 {x: 1, y: Cell::new(20) });
    }

}