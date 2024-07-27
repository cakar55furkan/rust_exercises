use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read from input");

    let mut number_guess = String::new();

    io::stdin()
        .read_line(&mut number_guess)
        .expect("failed to read from input");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Error at parsing string to integer");
    match guess.cmp(&secret_number) {
        Ordering::Greater => {
            println!("Greater");
            //something else
        }
        Ordering::Equal => println!("you guessed correctly"),
        Ordering::Less => println!("less"),
    }

    let mut counter: u32 = 0;

    loop {
        println!("{}", rand::thread_rng().gen_range(1..=500));
        if counter > 100 {
            break;
        }
        counter = counter + 1;
    }
}

fn alter_string(mut s: String) -> String {
    /*
    receives the ownership of the variable.
    */
    s.push_str(" appended text");
    return s;
}
