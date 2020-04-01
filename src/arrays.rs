

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn fixed_length_capacity() {
        let array: [i32; 3] = [0; 3]; // init val zero, capacity of 3
        assert_eq!([0, 0], &array[1..]);
        // array.
    }

    #[test]
    fn initialize_mutable() {
        let mut array: [i32; 3] = [0; 3];
        array[1] = 1;
        array[2] = 2;
        assert_eq!([1, 2], &array[1..]);
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

