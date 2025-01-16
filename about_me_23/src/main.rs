fn main() {
    /*
    Create a new `about_me` project with the `cargo new` command.  xx

    Using the `println!` macro, output 3 sentences about yourself. xx
    Feel free to invoke the macro multiple times.

    From the Terminal, compile the `main.rs` file inside the `src`
    folder with the Rust compiler, then manually run the executable. xx

    From the Terminal, compile the project with the Cargo tool, then
    manually run the executable.  xx

    From the Terminal, compile and run the project with a single
    Cargo command. xx

    Check your program for errors with `cargo check`. xx

    Add a comment at the top of your source code explaining how to
    compile the program for new Rust developers. xx

    Add some spaces and line breaks to the code so that it is formatted
    in an ugly manner. From the Terminal, style the code with the    xx
    `cargo fmt` command.                                             xx

    Replace the `println!` macro with `print!`. What happens? the lines are not separated by a newline! x
    */

    /*
    you can do this in different ways,
    cargo run  which compiles and runs the program
    cargo run --release which compiles and runs the program in release mode
    cargo build which compiles the program but does not run it
    cargo build --release which compiles the program in release mode but does not run it
    cargo build --debug which compiles the program in debug mode but does not run it
    cargo check which checks the program for errors but does not compile it
    cargo fmt which formats the code
     */
    print!("Hello, Person!");
    println!("I am a Software Developer");
    println!("I work with Python and I am learning Rust");
    println!("I am a student at the University of Student Högskolan, Stockholm");
    println!("My dream is to work in the field of AI and Machine Learning");
    println!("I live in Stockholm, Sweden and I love the nature");
    println!(
        "I am a huge fan of the Swedish table top role playing game system, Call of Cthulhu Sweden"
    );
    println!(
        "I love to play the guitar and I am learning to play the piano and I find it relaxing"
    );
    println!(
        "My favorite food is sushi and I love to eat out at restaurants with my friends and family"
    );
}
