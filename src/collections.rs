

// Vec<T>   - A heap-allocated vector that is resizable at runtime.
// [T; n]   - An inline array with a fixed size at compile time.
// [T]      - A dynamically sized slice into any other kind of contiguous storage,
//              whether heap-allocated or not.

pub fn collections() {
    map();

}

fn map() {
    let xyz = a.iter().map(|n| n.mul(2));
    println!("xyz: {:?}", xyz);

}