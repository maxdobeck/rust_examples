extern crate regex;

use regex::Regex;

fn found_email(text: &str) -> Option<&str> {
    // define the reg expression you're looking for
    let re = Regex::new(r"[[:word:]]+@[[:word:]]+\.[[:word:]]+").unwrap();

    // capture any matching expressions and
    re.find(text).map(|email_found| email_found.as_str())
}

fn main() {
    let sample: &str =
        "this is a long piece of text containing one email address: somebody@company.com";
    let email = found_email(sample);
    println!("{:?}", email)
}
