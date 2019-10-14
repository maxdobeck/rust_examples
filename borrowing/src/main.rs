use std::io;
use std::io::Write;

fn main() {
    // borrowing string example
    let s1 = String::from("hello!");
    let len = calculate_length(&s1);
    println!("the length of {} is {}", s1, len);

    // borrowing integer example
    let num1 = 23_000;
    println!("the value of the first number is {}", num1);
    println!("the value of the first number doubled is {}", double(num1));

    // getting input and borrowing it in different functions
    print!("{}", "Enter your text:".to_string());

    io::stdout().flush().unwrap();
    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Problem getting input");

    let clean_val = String::from(val.trim_end());

    println!("you entered: {}", clean_val);
    let input_len = calculate_length(&clean_val);
    println!("the length of {} is {}", clean_val, input_len);

    println!(
        "double the length of your input is {}",
        double(input_len as u64)
    )
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn double(n: u64) -> u64 {
    n * 2
}
