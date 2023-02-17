pub fn assert_match<T: PartialOrd>(arr1: &Vec<T>, arr2: &Vec<T>) {
    arr1.iter()
        .zip(arr2.iter())
        .for_each(|(a, b)| assert!(a == b, "values doesn't match"))
}
