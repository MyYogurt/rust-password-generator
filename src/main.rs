extern crate clap;
extern crate rand;
extern crate clipboard;
use clap::{App, Arg};
use rand::Rng;
use clipboard::{ClipboardProvider, ClipboardContext};

///Main function of program. Parses user arguments using clap and passes these arguments to the generate_password function
fn main() {
    let matches = App::new("rust-password-generator")
        .author("MyYogurt")
        .about("Generates passwords using Rust")
        .version("0.2")
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
            .takes_value(false)
            .help("Specify true if you don't want the password to be shown. Default: password shown"))
        .arg(Arg::with_name("numbers")
            .short("n")
            .long("numbers")
            .value_name("NUMBERS")
            .takes_value(true)
            .help("Specifies whether a password must have at least one number or must not have numbers. true to require at least one number, false to require no numbers. Default: numbers allowed but no required."))
        .arg(Arg::with_name("copy to clipboard")
            .short("c")
            .long("copy")
            .value_name("COPY")
            .takes_value(false)
            .help("Specifies whether the password should be copied to the user's keyboard. Default: false"))
        .get_matches();
    //Parse for password length
    let mut password_length: usize = 8; //default of 8 if no argument passed
    if let Some(o) = matches.value_of("length") {
        match o.parse::<usize>() {
            Ok(y) => password_length = y,
            Err(_) => println!("Length must be an integer. Defaulting to 8"),
        }
    }
    //Parse for special characters argument
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
    //Parse for don't show argument
    let mut dont_show: bool = false;
    if matches.occurrences_of("don't show") >= 1 {
        dont_show = true;
    }
    //Parse for numbers argument
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
    //Parse for copy to clipboard argument
    let mut copy_to_clipboard: bool = false;
    if matches.occurrences_of("copy to clipboard") >= 1{
        copy_to_clipboard = true;
    }
    //Copy password to clipboard if requested
    if copy_to_clipboard {
        let mut clip: ClipboardContext = ClipboardProvider::new().unwrap();
        clip.set_contents(password.clone()).unwrap();
    }
    //Check if user requested that their password not be printed to the screen
    if !dont_show {
        println!("Your password: {}", password);
    }
}

///Genertes a password that complies with the provided parameters
/// # Parameters
/// 
/// * `length` - size of password
/// * `characters` - -1 for no special characters, 0 for special characters to be allowed but not required, 1 for at least one special character
/// * `numbers` - -1 for no numbers in password, 0 for numbers to be allowed but not required, 1 for at least one number in password
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
    //Check if user requested at least one number to be in the password and if the current password complies with that request
    if numbers == 1 && !contains_number(&string_builder) {
        let integers = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
        let position = rng.gen_range(0, string_builder.len());
        string_builder[rng.gen_range(0, position)] = integers[rng.gen_range(0, integers.len())];
    }
    //Check if user requested at least one special character to be in the password and if the current password complies with that request
    if characters == 1 && !contains_special_character(&string_builder) {
        let special_characters = ['`', '~', '!', '#', '$', '%', '^', '&', '*', '(', ')', '[', ']', ':', ';', '"', '\'', '<', '>', ',', '.', '/', '?'];
        let position = rng.gen_range(0, string_builder.len());
        string_builder[rng.gen_range(0, position)] = special_characters[rng.gen_range(0, special_characters.len())];
    }

    string_builder.into_iter().collect::<String>()
}

///Checks if the vector has at least one numeric char
fn contains_number(password: &Vec<char>) -> bool {
    for x in password {
        if x.is_numeric() {
            return true
        }
    }
    false
}

///Checks if the vector has at least on special character
fn contains_special_character(password: &Vec<char>) -> bool {
    let special_characters = ['`', '~', '!', '#', '$', '%', '^', '&', '*', '(', ')', '[', ']', ':', ';', '"', '\'', '<', '>', ',', '.', '/', '?'];
    for x in password {
        if special_characters.contains(&x) {
            return true
        }
    }
    false
}