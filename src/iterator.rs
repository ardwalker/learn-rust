


pub fn iterator() {
    let a = [1, 2, 3];
    assert!(a.iter().all(|&x| x > 0));
    assert!(a.iter().any(|&x| x > 0));
    collect();
}

fn collect() {
    let a = [1, 2, 3];
    let doubled: Vec<i32> = a.iter()
        .map(|&x| x * 2)
        .collect();

    assert_eq!(vec![2, 4, 6], doubled);
}