fn main() {
    // Scalar types:
    // Integer
    // Floating-point numbers
    // Booleans
    // Characters

    let number:u8 = 250;

    println!("Number is: {}!", number);

    // Floating point precision
    let x = 2.0; // f64
    let y:f32 = 3.0; // f32

    // tuples
    let tup = (200, 40, 1.5);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
}
