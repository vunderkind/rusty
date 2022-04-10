fn main() {
    // Scalar types:
    // Integer
    // Floating-point numbers
    // Booleans
    // Characters

    let number:u8 = 250;

    println!("Number is: {}!", number);

    // Floating point precision
    let _x = 2.0; // f64
    let _y:f32 = 3.0; // f32

    // tuples
    let tup = (200, 40, 1.5);
    let (x, _, _) = tup;
    println!("The value of x is: {}", x);

    // Array: fixed length, same type
    let arr:[i32;5] = [1, 2, 3, 4, 5];

    println!("The value of arr[0] is: {}", arr[0]);

    // Next up: borrowing
}
