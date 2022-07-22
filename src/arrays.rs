

#[cfg(test)]
mod tests {

    #[test]
    fn array_literals_init() {
        let a1: [i32; 3]  = [0; 3]; // init val zero, capacity of 3
        let a2: [u8; 3]   = [1, 2, 3];
        let a3: [&str; 3] = ["1", "2", "3"];        
        let a4: [String; 3] = [String::from("1"),String::from("2"),String::from("3")];
    
        assert_eq!([0, 0, 0], a1);
        assert_eq!([1, 2, 3], a2);
        assert_eq!(["1", "2", "3"], a3);
        assert_eq!(["1", "2", "3"], a4);
    }
    

    #[test]
    fn access_array_elements() {
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];
        assert_eq!(first, 1);
        assert_eq!(second, 2);
    }


    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn invalid_array_access() {
        let a = [1, 2, 3, 4, 5];
        let index = 10;
    
        let _element = a[index];
    }

    #[test]
    fn initialize_mutable() {
        let mut array: [i32; 3] = [0; 3];
        array[1] = 1;
        array[2] = 2;
        assert_eq!([0, 1, 2], array);
    }

    #[test]
    fn initialize_default() {
        let array_implied_type = [10,20,30,40];
        assert_eq!([10, 20, 30, 40], array_implied_type);

        let array2:[i32;4] = [10,20,30,40];
        assert_eq!([10, 20, 30, 40], array2);
    }

    #[test]
    fn array_iteration() {
        let arr:[i32;4] = [10,20,30,40];
        let mut acc = 0;

        for val in arr.iter() { acc += *val; }
        assert_eq!(acc, 100);
    }

    #[test]
    fn pass_by_value() {
        let arr = [10,20,30];
        update(arr);
        fn update(mut array: [i32;3]) { array[0] = 50; }
        assert_eq!(10, arr[0]); // Not changed, because we passed by value
    }


    #[test]
    fn pass_by_reference() {
        let mut arr = [10,20,30];
        update(&mut arr);
        fn update(array : &mut [i32;3]) { array[0] = 50; }
        assert_eq!(50, arr[0]);
    }

    #[test]
    fn slice_pattern() {
        let [john, roa] = ["John".to_string(), "Roa".to_string()];
        assert_eq!(john, "John");
        assert_eq!(roa, "Roa");
    }

    // #[test]
    // fn using_macro() {
    //     let arr = arr!["A"; 33];
    // }
}

