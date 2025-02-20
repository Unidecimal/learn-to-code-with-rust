const TAX_RATE: f64 = 7.25;
const TOUCHDOWN_POINTS: i32 = 6;
type Meters = i32; 

fn main() {
    let width = get_terminal_width();
    variables(width);
    mutability(width);
    rust_error_codes_index(width);
    variable_shadowing(width);
    variables_and_scopes(width);
    constants(width);
    type_aliases(width);
    compiler_directives(width);

    // PROJECT:
    project(width);

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


fn variables_and_scopes(width: usize) {
    print_headline(width, "variables scope");
    
    // this is a practical example of a second independent variable with the same name in the nested scope.
    let coffee_price = 10;
    { // this a nested scope, the variable x is declared again in the inner scope.
        let coffee_price = 20;
        let cookie_price = 5;
        println!("coffee_price is {coffee_price}");
        println!("the cookie price is {cookie_price}");
    }
    println!("coffee_price is {coffee_price}");
    println!("this will cause an error because the cookie_price variable is not in the scope of the outer block.")
    // println!("the cookie price is {cookie_price}"); 
}

fn constants(width: usize) {
    print_headline(width, "constants");
    let income: i32 = 1_000_000;
    println!("constants are like variables but they are immutable and cannot be changed.");
    println!("constants have the scoope of the entire program and can thus be declared in"); 
    println!("the file level outside of any function (eg: at the top of the file outside of main).");
    println!("constants are declared using the const keyword instead of let.");
    println!("constants must be annotated with a type.");
    println!("My in come is {income} and the TAX_RATE is {TAX_RATE}");

}

fn type_aliases(width: usize) {
    print_headline(width, "type aliases");
    println!("type aliases are a way to give a different name to a type.");
    println!("they are declared using the type keyword.");
    println!("I declare 'type Meters = i32;' on top of the file to use it in the code.");
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = mile_race_length * 2;
    let marathon_length: Meters = 42_195;
    println!("The length of a mile race is {mile_race_length} meters.");
    println!("The length of a two mile race is {two_mile_race_length} meters.");
    println!("The length of a marathon is {marathon_length} meters.");  
}

#[allow(unused_variables)]
fn compiler_directives(width: usize) {
    print_headline(width, "type aliases");
    println!("A compiler directive is a instruction to the compiler how to parse the source code");
    println!("add #[allow(unused_variables)] - to allow unused variables if you adding it to a function.");
    println!("it will allow unused variables in the whole function.");
    println!("if you look in the code in the 'compiler_directives' function, you will see that all the...");
    println!("Unused variables are allowed. and the IDE will not show any errors.");
    println!("If you want to allow unused variables in the whole file you can declare it at the top of the file.");
    println!("But then you will have to add an exlamation mark to the hashtag."); 
    println!("like this: #![allow(unused_variables)]");
    let marathon_length: Meters = 42_195;
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = mile_race_length * 2;
}

fn project(width: usize) {
    /* PROJECT:   
    Declare a `season` variable set to a string with
    your favorite season. Provide an explicit type annotation.
    The type of a string is a `&str`. We'll discuss what
    the & symbol means later in the course.
    
    Declare a `points_scored` variable set to 28.
    Provide an explicit type annotation. The type of
    an integer is `i32`.
    
    It's time to update the team's score. Declare the
    `points_scored` variable to be mutable. Set its
    new value to 35.
    
    Declare a `TOUCHDOWN_POINTS` constant at the file
    level set to the value 6.
    
    Declare a `event_time` variable set to a string of
    "06:00".
    
    Use variable shadowing to redeclare `event_time` set
    to a integer of 6.
    
    Use interpolation to print out all of the
    declared variables and constants in a println! call.
    Practice using direct interpolation {value}, sequential
    arguments ( {} ), and numeric arguments ( {0} ).
    
    Declare a `favorite_beverage` variable set to a string
    of your favorite drink. Use an underscore to silence
    the compiler warning about the variable being unused.
    
    Remove the underscore. Provide a compiler directive
    to silence the compiler warning about the variable
    being unused.
    */

    let season: &str = "winter";
    let points_scored: i32 = 28;
    let mut points_scored: i32 = 35;
    // const TOUCHDOWN_POINTS: i32 = 6; declared at the top of the file.
    let event_time: &str = "06:00";
    let event_time: i32 = 6;
    println!("The season is {season}, the points scored are {points_scored}, the touchdown points are {TOUCHDOWN_POINTS}, the event time is {event_time}.");
    println!("The season is {0}, the points scored are {1}, the touchdown points are {2}, the event time is {3}.", season, points_scored, TOUCHDOWN_POINTS, event_time);
    let _favorite_bevarege: &str = "sara";
    #[allow(unused_variables)]
    let favorite_bevarege: &str = "aras";
}


fn print_headline(width: usize, text: &str) {
    let padding_size = (width / 2) - (text.len() / 2);
    println!("{0}{text}{0}", "*".repeat(padding_size));
}