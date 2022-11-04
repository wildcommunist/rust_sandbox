// Point to memory reference
pub fn run() {
    // Primitive array. No issue pointing to them
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = arr1;

    println!("values: {:?}", (arr1, arr2));

    // Vector - non-primitive types, you need to create reference, otherwise the value will not be moved!
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = &vec1;

    println!("values: {:?}", (&vec1, vec2));
}