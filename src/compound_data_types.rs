fn main() {
    // arrays, tuples, slices and strings (slice string)

    // Arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array at 2nd index: {:?}", fruits[2]);

    // Tuples
    let tup: (i32, &str, bool) = (1, "hello", true);
    println!("Tuples: {:?}", tup);

    // Slices
    let slice: &[i32] = &arr[1..3];
    println!("Slices: {:?}", slice);

    // Strings
    let string: &str = "hello";
    println!("Strings: {:?}", string);
}
