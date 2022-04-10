pub fn run() {
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("{:?}", (arr1, arr2));

    let arr3 = vec![1,2,3];
    let arr4 = &arr3;

    println!("{:?}", (&arr3, arr4));
}