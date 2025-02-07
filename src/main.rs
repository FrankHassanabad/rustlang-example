use cipher_text::CipherText;
use options::{CmdOptions, CmdOptionsError};
use std::env;
use std::io::{self};

mod cipher_text;
mod options;

// char set to determine if the crib looks to have uncovered something
const CHAR_SET: &str = "a-zA-Z0-9.,?! :;\'\"";

// This is the encoded text which has been encrypted with a flawed xor cipher.
const DEFAULT_CIPHER_TEXT: &str = "00000052480f131d545712180e1544571d0409001204541c4f4f054f1a1c0443175215110303491008040b0a1b4e2109094110035b430e1f45541d4513011a450f191700121d175406000a54491e0e0e0108411b0f4f1a4645630c06061e0d111d494b0064";

// The message to reveal if the user asks for it
const MESSAGE_REVEALED: &str = "You have walked the path of the cryptographer and you are ready for further information. Congrats.... Your next step will be to contact lawl.bitcoin@gmail.com to receive the next phase of the challenge.";

fn main() -> io::Result<()> {
    let src_ciper_text = env::args().nth(1);
    let mut cypher_text = match src_ciper_text {
        Some(text) => CipherText::new(text, CHAR_SET.to_string()),
        None => CipherText::new(DEFAULT_CIPHER_TEXT.to_string(), CHAR_SET.to_string()),
    }
    .expect("Failed to create CipherText");
    options::print_intro();
    options::print_options()?;
    startup(&mut cypher_text)
}

fn startup(cypher_text: &mut CipherText) -> io::Result<()> {
    let option = options::get_option();
    match option {
        Ok(CmdOptions::CribTry(input)) => {
            cypher_text.enter_crib_try(&input);
            options::print_options()?;
            startup(cypher_text)
        }
        Ok(CmdOptions::PrintHistory) => {
            cypher_text.print_history()?;
            options::print_options()?;
            startup(cypher_text)
        }
        Ok(CmdOptions::Hint) => {
            options::print_hint();
            options::print_enter_option()?;
            startup(cypher_text)
        }
        Ok(CmdOptions::Reveal) => {
            println!("The message is: {}", MESSAGE_REVEALED);
            options::print_enter_option()?;
            startup(cypher_text)
        }
        Ok(CmdOptions::Exit) => Ok(()),
        Err(CmdOptionsError::Invalid(i8)) => {
            println!("Invalid option of: {}", i8);
            options::print_enter_option()?;
            startup(cypher_text)
        }
        Err(CmdOptionsError::IoError(err)) => Err(err),
    }
}
