extern crate regex;

use regex::Regex;

fn found_email(text: &str) -> Option<&str> {
    // define the reg expression you're looking for
    let re = Regex::new(r"[[:word:]]+@[[:word:]]+\.[[:word:]]+").unwrap();

    // capture any matching expressions and
    re.find(text).map(|email_found| email_found.as_str())
}

fn main() {
    // No unwrap vs unwrapping the Some() return
    let sample: &str =
        "this is a long piece of text containing one email address: ahumanbeing@someorg.horse";
    let some_email = found_email(sample);
    println!("//unwrapping the Some() return vs No unwrap");
    print!("{} vs {:?}\n\n", some_email.unwrap(), some_email);

    // Printing {:?} Some or None objects vs Panicking while trying to .unwrap()
    let sample_missing_email: &str =
        "this is a long piece of text containing one email address: whoops-Email-has-been-corrupted!";
    let missing_email = found_email(sample_missing_email);
    println!("// Printing Some or None objects vs Panicking while trying to .unwrap()");
    println!("{:?}\n", missing_email);
    // this will Panic!
    //println!("{}", missing_email.unwrap());

    // check if value is_some() and print it
    let sample: &str =
        "this is a long piece of text containing one email address: somebody@company.com";
    let email = found_email(sample);
    println!("// check if value is_some() and print it");
    if email.is_some() {
        println!("{}\n", email.unwrap());
    }

    // if let and print the unwrapped value
    let sample2: &str = "Reach me at aperson@somebusiness.com.";
    println!("// if let and print the unwrapped value");
    if let Some(v) = found_email(sample2) {
        println!("{:?}\n", v)
    }

    // if let with an option for the None
    let sample3: &str = "There is no email to be found here!";
    let no_email = found_email(sample3);
    println!("// if let with an option for the None");
    if let Some(i) = no_email {
        println!("{:?}\n", i)
    } else {
        println!("Could not find an email")
    }
}
