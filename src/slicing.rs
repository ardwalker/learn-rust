
// &[T]     - shared slice
// &mut [T] - mutable slice
// Box<[T]> - owned slice

// Slicing -----------------------------------------------------
pub fn slicing() {
    slicing_arrays();
    slicing_and_references();
}

fn slicing_arrays() {
    println!("-> Slicing arrays");
    let a: [i32; 4] = [1, 2, 3, 4]; // Parent Array
    let b: &[i32] = &a; // Slicing whole array
    let c = &a[0..4]; // From 0th position to 4th(excluding)
    let d = &a[..]; // Slicing whole array
    let e = &a[1..3]; // [2, 3]
    let f = &a[1..]; // [2, 3, 4]
    let g = &a[..3]; // [1, 2, 3]

    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}, e: {:?}, f: {:?}, g: {:?}", a, b, c, d, e, f, g);
}

// [] is slice
fn slicing_and_references() {
    println!("-> Slicing and references");

    let v: Vec<f64> = vec![0.0,  0.707,  1.0,  0.707];  //  vector (stack)
    let a: [f64; 4] =     [0.0, -0.707, -1.0, -0.707];  //  array (heap)

    // Rust automatically converts the vector and array references
    // to slice references...
    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    prt(sv);
    prt(sa);

}

// This function accepts a slice of type f64
fn prt(a: &[f64]) {
    println!("Prints array or vec slice ref {:?}", a);
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn slicing_arrays() {
        let a: [i32; 4] = [1, 2, 3, 4]; // Parent Array
        let b: &[i32] = &a;             // Slicing whole array
        assert_eq!(b, [1,2,3,4]);
    }

    #[test]
    fn slicing_array_range() {
        let arr: [i32; 4] = [1, 2, 3, 4]; // Parent Array        
        let c = &arr[0..4]; // From 0th position to 4th(excluding)
        let d = &arr[..];   // Slicing whole array
        let e = &arr[1..3]; // [2, 3]
        let f = &arr[1..];  // [2, 3, 4]
        let g = &arr[..3];  // [1, 2, 3]

        assert_eq!(c, [1,2,3,4]);
        assert_eq!(d, [1,2,3,4]);
        assert_eq!(e, [2,3]);
        assert_eq!(f, [2,3,4]);
        assert_eq!(g, [1,2,3]);
    }

    #[test]
    fn string_slice() {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        assert_eq!(hello, "hello");
        assert_eq!(world, "world");
    }
}