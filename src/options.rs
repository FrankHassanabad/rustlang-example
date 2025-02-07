use rand::Rng;
use std::io::{self, Write};

/// Print the introduction to the program.
/// # Example
/// ```
/// print_intro();
/// ```
pub fn print_intro() {
    println!(
        "Welcome to the Cipher Cracker\n\
         ----\n\
         This program will help you crack a cipher that has been encoded with a fatal flaw. By having an initial\n\
         piece of information such as a word or phrase, you can use this information to help crack parts of the cipher.\n\
         Text you use to try and crack the Cipher is called a 'crib' and the techniques used under the hood are XOR strings.\n\
         For example, if you know that the word 'cryptographer' is in the encoded text, you can use this word as a crib to help crack the cipher.\n\
         After each word(s) you try, a set of 'chunks' of text will be displayed to help see if you decoded a different part of the message.\n\
         If you have decoded a different part of the message you can optionally use that text or complete any word(s) or phrase to crack more of the message.\n\
         The output might have tabs, spaces, or new line characters and is expected to be messy as you try to extract information from the message.\n\
         It is expected that chunks will be displayed which are not part of the message as well.\n\
         Start by typing in the word 'cryptographer' and see what happens.\n\
         See https://samwho.dev/blog/toying-with-cryptography-crib-dragging/ for more information.\n\
         ----");
}

/// Print the available options to the console.
/// # Example
/// ```
/// print_options();
/// ```
pub fn print_options() -> io::Result<()> {
    println!(
        "\nAvaialble Options:\n\
                Enter any ascii character for a crib try (e.x. cryptographer)\n\
                Enter 1 to print history of crib attempts you have tried so far\n\
                Enter 2 for a hint\n\
                Enter 3 to reveal the message\n\
                Enter 4 to exit"
    );
    print_enter_option()
}

/// Print the prompt to enter an option.
/// # Example
/// ```
/// print_enter_option();
/// ```
pub fn print_enter_option() -> io::Result<()> {
    print!("Enter option: ");
    io::stdout().flush()
}

/// Print a hint to the console. The hint is randomly selected from a set of hints.
/// # Example
/// ```
/// print_hint();
/// ```
pub fn print_hint() {
    let mut rng = rand::rng();
    let random_number = rng.random_range(1..=5);
    match random_number {
        1 => println!("Hint: enter the word 'Your next step'"),
        2 => println!("Hint: enter the words 'next phase'"),
        3 => println!("Hint: enter the word 'cryptographer'"),
        4 => println!("Hint: enter the words 'You have walked the path'"),
        5 => println!("Hint: enter the word 'further information'"),
        _ => println!("Hint: enter the word 'lawl.bitcoin@gmail.com'"),
    }
}

/// The options available to the user.
#[derive(Debug, PartialEq)]
pub enum CmdOptions {
    CribTry(String),
    PrintHistory,
    Hint,
    Reveal,
    Exit,
}

/// The error that can occur when getting the option from the user input.
#[derive(Debug)]
pub enum CmdOptionsError {
    Invalid(i8),
    IoError(std::io::Error),
}

impl From<std::io::Error> for CmdOptionsError {
    fn from(err: std::io::Error) -> CmdOptionsError {
        CmdOptionsError::IoError(err)
    }
}

/// Get the option from the user input.
/// # Example
/// ```
/// let option = options::get_option();
/// match option {
///    Ok(CmdOptions::CribTry(input)) => {
///        cypher_text.enter_crib_try(&input);
///        options::print_options()?;
///        startup(cypher_text)
///    }
///    Ok(CmdOptions::PrintHistory) => {
///        cypher_text.print_history();
///        options::print_options()?;
///        startup(cypher_text)
///    }
///    Ok(CmdOptions::Hint) => {
///        options::print_hint();
///        options::print_enter_option()?;
///        startup(cypher_text)
///   }
///    Ok(CmdOptions::Reveal) => {
///        println!("The message is: {}", MESSAGE_REVEALED);
///        options::print_enter_option()?;
///        startup(cypher_text)
///    }
///    Ok(CmdOptions::Exit) => Ok(()),
///    Err(CmdOptionsError::Invalid(u32)) => {
///        println!("Invalid option of: {}", u32);
///        options::print_enter_option()?;
///        startup(cypher_text)
///    }
///    Err(CmdOptionsError::IoError(err)) => Err(err),
/// }
/// ```
pub fn get_option() -> Result<CmdOptions, CmdOptionsError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    get_option_from_input(&input)
}

fn get_option_from_input(input: &str) -> Result<CmdOptions, CmdOptionsError> {
    let maybe_number = input.trim().parse::<i8>();
    match maybe_number {
        Ok(1) => Ok(CmdOptions::PrintHistory),
        Ok(2) => Ok(CmdOptions::Hint),
        Ok(3) => Ok(CmdOptions::Reveal),
        Ok(4) => Ok(CmdOptions::Exit),
        Err(_) => Ok(CmdOptions::CribTry(input.to_string())),
        Ok(n) => Err(CmdOptionsError::Invalid(n)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_option_from_input_print_history() {
        let input = "1";
        let result = get_option_from_input(input);
        assert!(matches!(result, Ok(CmdOptions::PrintHistory)));
    }

    #[test]
    fn test_get_option_from_input_hint() {
        let input = "2";
        let result = get_option_from_input(input);
        assert!(matches!(result, Ok(CmdOptions::Hint)));
    }

    #[test]
    fn test_get_option_from_input_reveal() {
        let input = "3";
        let result = get_option_from_input(input);
        assert!(matches!(result, Ok(CmdOptions::Reveal)));
    }

    #[test]
    fn test_get_option_from_input_exit() {
        let input = "4";
        let result = get_option_from_input(input);
        assert!(matches!(result, Ok(CmdOptions::Exit)));
    }

    #[test]
    fn test_get_option_from_input_invalid_pos_num() {
        let input = "5";
        let result = get_option_from_input(input);
        assert!(matches!(result, Err(CmdOptionsError::Invalid(5))));
    }

    #[test]
    fn test_get_option_from_input_invalid_neg_num() {
        let input = "-1";
        let result = get_option_from_input(input);
        assert!(matches!(result, Err(CmdOptionsError::Invalid(-1))));
    }

    #[test]
    fn test_get_option_from_input_crib_try() {
        let input = "some crib try";
        let result = get_option_from_input(input);
        assert!(matches!(result, Ok(CmdOptions::CribTry(ref s)) if s == input));
    }
}
