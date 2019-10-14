use std::io;
use std::io::Write;

fn main() {
    let input = get_input();
    println!("{}", input);
}

fn get_input() -> String {
    // print!("Enter your text: ");
    // Alternative to printing literal text.  Use an IMMUTABLE variable instead!
    let prompt: String;
    prompt = "Enter your text:".to_string();
    print!("{}", prompt);

    // reuse a mutable variable
    // let mut prompt = "Enter your text:".to_string();
    // prompt = "What do you want to echo?".to_string();
    // print!("{}", prompt);

    // shadow an immutable variable
    // let prompt = "Enter your text: ".to_string();
    // let prompt = "What do you want to echo? ".to_string();
    // print!("{}", prompt);

    // output 42 as an example of shadowing
    // let prompt: String = "Enter your text: ".to_string();
    // let prompt = 42;
    // print!("{}", prompt);


    io::stdout().flush().unwrap();
    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Error getting input");

    return val;
}
