fn main() {
    let width = get_terminal_width();
    variables(width);
    mutability(width);
    rust_error_codes_index(width);
    variable_shadowing(width);
}

fn get_terminal_width() -> usize {
    use terminal_size::{Width, terminal_size};
    let size = terminal_size();
    if let Some((Width(w), _)) = size {
        w as usize
    } else {
        80  // fallback
    }
}

fn mutability(width: usize) {
    print_headline(width, "mutability");
    let mut gym_reps: i32 = 10; // mut is used to make the variable / binding mutable.
    println!("I plan to do {gym_reps} reps");

    gym_reps = 15;
    println!("I plan to do {gym_reps} reps");
}

fn variables(width: usize) {
    print_headline(width, "variables");
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

fn rust_error_codes_index(width: usize) {
    print_headline(width, "rust error codes index");
    println!("When getting a error code, For more information about this error, try `rustc --explain E0384` in the terminal.");
    println!("There is also a website called doc.rust-lang.org/error_codes/index.html that can help you understand the error code.");
    println!("With formated code examples and even possility to run the code in the browser to see what is happening.");
}

fn variable_shadowing(width: usize) {
    print_headline(width, "variable shadowing");
    println!("Variable shadowing means redeclaring a variable. The original variable is replaced by the new one.");
    let grams_of_protein: &str  = "110.135";
    println!("Grams of protein as String: {grams_of_protein}");
    let grams_of_protein: f32 = 110.135; // imagine we are extracting a number from a string and returning a float.
    println!("Grams of protein as float 32: {grams_of_protein}");
    // when we declare the variable with the 'let' keyword again, the original variable is replaced by the new one.
    // this is different from mutability, which allows us to change the value of a variable without redeclaring it 
    // but it must be of the same type which is not practical in this case, we are imagining 'extracting' a number
    // from a string. so we need to declare the variable again with the new type.
    // this is a practical example of variable shadowing.
    let mut grams_of_protein: i32 = 110; // imagine we are converting and returning an mutable integer.
    println!("Grams of protein as Integer: {}", grams_of_protein);
    grams_of_protein = 120;
    println!("Grams of protein as mutated Integer: {}", grams_of_protein);
}


fn print_headline(width: usize, text: &str) {
    let padding_size = (width / 2) - (text.len() / 2);
    println!("{0}{text}{0}", "*".repeat(padding_size));
}