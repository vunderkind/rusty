fn main() {
    let mut x = 5;
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The number of seconds in three hours is: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing experiment
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    let spaces = "          ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);
}
