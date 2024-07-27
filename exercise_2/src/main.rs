fn main() {
    /*
     * Exercises variables types in Rust Book chapter 2
     * */
    const MINUTES_IN_HOURS: u32 = 60;

    println!("Minutes in hours: {}", MINUTES_IN_HOURS);
    let x = 10;

    println!("Initial value of x: {}", x);

    {
        // shadowing inside of the scope
        let x = x * 3;
        println!("Value of x in inner scope {}", x);
    }
    println!("Value of x after exiting the scope: {}", x);

    let x = x - 5;
    println!("Value of x after shadowing outside of the loop: {}", x);
    // shadowing creates another variable with different value.
    // Even though it is an immutable variable.

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    // Data Types:
    // Data type of a variable in Rust must be determined at compile-time

    let mut var = 55;
    let int_from_str: u32 = "123".parse().expect("Error parsing string to int");
    println!("{}", int_from_str);
    // let positive_number: u32 = -5;
    // print!("{}", positive_number);
}
