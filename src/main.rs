extern crate clap;
extern crate rand;
use clap::{App, Arg};
use rand::Rng;

fn main() {
    let matches = App::new("rust-password-generator")
        .author("MyYogurt")
        .about("Generates passwords using Rust")
        .version("0.1")
        .arg(Arg::with_name("length")
            .short("l")
            .long("length")
            .value_name("LENGTH")
            .help("Sets the length of the password. Defualt is 8 characters.")
            .takes_value(true))
        .arg(Arg::with_name("special characters")
            .short("s")
            .long("specialcharacters")
            .value_name("SPECIAL CHARACTERS")
            .takes_value(true)
            .help("Specifies whether a password must have a special character or must not have a special character. true to require a special character, false to require no special characters. Default: special characters allowed but not required."))
        .arg(Arg::with_name("don't show")
            .short("d")
            .long("dontshow")
            .value_name("DON'T SHOW")
            .takes_value(true)
            .help("Specify true if you don't want the password to be shown. Default: password shown"))
        .arg(Arg::with_name("numbers")
            .short("n")
            .long("numbers")
            .value_name("NUMBERS")
            .takes_value(true)
            .help("Specifies whether a password must have at least one number or must not have numbers. true to require at least one number, false to require no numbers. Default: numbers allowed but no required."))
        .get_matches();
    let mut password_length: usize = 8;
    if let Some(o) = matches.value_of("length") {
        match o.parse::<usize>() {
            Ok(y) => password_length = y,
            Err(_) => println!("Length must be an integer. Defaulting to 8"),
        }
    }
    let mut special_characters: i8 = 0;
    if let Some(o) = matches.value_of("special characters") {
        if o.trim() == "true" {
            special_characters = 1;
        }
        else if o.trim() == "false" {
            special_characters = -1;
        }
        else {
            println!("Invalid input for special characters. Setting to default: special characters allowed but not required.");
        }
    }
    let mut dont_show: bool = false;
    if let Some(o) = matches.value_of("don't show") {
        if o.trim() == "true" {
            dont_show = true;
        }
    }
    let mut numbers: i8 = 0;
    if let Some(o) = matches.value_of("numbers") {
        if o.trim() == "true" {
            numbers = 1;
        }
        else if o.trim() == "false" {
            numbers = -1;
        }
        else {
            println!("Invalid input for numbers. Setting to default: numbers allowed but not required.");
        } 
    }
    let password = generate_password(password_length, special_characters, numbers);
    if !dont_show {
        println!("Your password: {}", password);
    }
}

fn generate_password(length: usize, characters: i8, numbers: i8) -> String {
    let mut string_builder: Vec<char> = Vec::<char>::new();
    let mut rng = rand::thread_rng();
    let mut possibilities: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    if characters != -1 {
        possibilities.extend(&['`', '~', '!', '#', '$', '%', '^', '&', '*', '(', ')', '[', ']', ':', ';', '"', '\'', '<', '>', ',', '.', '/', '?']);
    }
    if numbers != -1 {
        possibilities.extend(&['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);
    }
    for _ in 0..length {
        let character: usize = rng.gen_range(0, possibilities.len());
        string_builder.push(possibilities[character]);
    }
    if numbers == 1 && !contains_number(&string_builder) {
        let integers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
        let position = rng.gen_range(0, string_builder.len());
        string_builder[rng.gen_range(0, position)] = integers[rng.gen_range(0, integers.len())];
    }
    if characters == 1 && !contains_special_character(&string_builder) {
        let special_characters = ['`', '~', '!', '#', '$', '%', '^', '&', '*', '(', ')', '[', ']', ':', ';', '"', '\'', '<', '>', ',', '.', '/', '?'];
        let position = rng.gen_range(0, string_builder.len());
        string_builder[rng.gen_range(0, position)] = special_characters[rng.gen_range(0, special_characters.len())];
    }

    string_builder.into_iter().collect::<String>()
}

fn contains_number(password: &Vec<char>) -> bool {
    for x in password {
        if x.is_numeric() {
            return true
        }
    }
    false
}

fn contains_special_character(password: &Vec<char>) -> bool {
    let special_characters = ['`', '~', '!', '#', '$', '%', '^', '&', '*', '(', ')', '[', ']', ':', ';', '"', '\'', '<', '>', ',', '.', '/', '?'];
    for x in password {
        if special_characters.contains(&x) {
            return true
        }
    }
    false
}