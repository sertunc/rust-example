fn main() {
    println!("Hello, Rust from CARGO!");

    // Rust has signed (+ and -) and unsigned integer (only+) types of different sizes.
    // i8, i16, i32, i64, i128: signed integers
    // u8, u16, u32, u64, u128: unsigned integers

    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer:{}", x);
    println!("Unsigned Integer:{}", y);
    // diff bet i32 (32 bits) and i64(64 bites)
    // i32 - 2147483648 to 2147483647
    // u64 - 9223372036854775808

    // Floats [Floating Point Types]
    // f32, f64
    let pi: f64 = 3.1415926535897932384626433832795028841971693993751;
    println!("Pi:{}", pi);

    // Boolean Values: true and false
    let is_true: bool = true;
    let is_false: bool = false;
    println!("Boolean:{}", is_true);
    println!("Boolean:{}", is_false);

    // Character Type - char
    let my_char: char = 'A';
    println!("Character:{}", my_char);

    
}
