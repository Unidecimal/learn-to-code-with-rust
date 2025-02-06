fn main() {
    use terminal_size::{Width, terminal_size};
    let size = terminal_size();
    let width = size.Width();
    variables();
    mutability();
    tips();
}

fn mutability() {
    println!("Mutability");
    let mut gym_reps: i32 = 10; // mut is used to make the variable / binding mutable.
    println!("I plan to do {gym_reps} reps");

    gym_reps = 15;
    println!("I plan to do {gym_reps} reps");
}

fn variables() {
    println!("variables");
    let apples_in_garden: i32 = 50;
    let apples_in_basball_park: i32 = 100;
    let oranges: i32 = 15 + 6;
    let fruits: i32 = apples_in_garden + apples_in_basball_park + oranges;
    let _banana: i32 = 10; // _ is used to indicate that the variable is not used.

    println!("Apples in garden: {}", apples_in_garden);
    println!("Apples in basball park: {}", apples_in_basball_park);
    println!("Total number of fruits {}", fruits); // {} is a placeholder for the value of fruits
    // the basic term for this is interpolation eg: in the string we interpolate the dynamic value of fruits.

    println!("This year, my garden has {apples_in_garden} apples and {} oranges but ten of them was rotten", oranges - 10);
    // as you can we can write the variables directly in the string inside of the curly braces. 

    println!("This year, my garden has {0} apples and {1} oranges but ten of them was rotten, i cant believe tha i got {0} apples.", apples_in_garden,oranges - 10);
    // we can also use the index of the variables in the string. That allow us to use the same variable in the string multiple times
    // without having to repeat the variable name.
}

fn tips() {
    println!("{0}tips{0}", "*".repeat(10));
    println!("When getting a error code, For more information about this error, try `rustc --explain E0384` in the terminal.");
    println!("There is also a website called doc.rust-lang.org/error_codes/index.html that can help you understand the error code.");
    println!("With formated code examples and even possility to run the code in the browser to see what is happening.");
}