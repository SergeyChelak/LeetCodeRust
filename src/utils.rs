pub fn assert_match(arr1: &Vec<i32>, arr2: &Vec<i32>) {
    arr1.iter()
        .zip(arr2.iter())
        .for_each(|(a, b)| assert_eq!(a, b, "values doesn't match"))
}
