use regex::Regex;
use std::io::{self, Write};
use std::num::ParseIntError;

/// CipherText holds the history of crib attempts, the cipher text that is being cracked and a regex which is used
/// to determine if the crib attempt has uncovered a possible match. When the regex finds a match, the text is printed
/// with a *** to indicate a possible match.
#[derive(Debug)]
pub struct CipherText {
    history: Vec<String>,
    cipher_text: String,
    regex_match_set: Regex,
}

impl CipherText {
    /// Create a new CipherText instance with the given cipher text and character set.
    /// # Example
    /// ```
    /// let input = "some phrase to try and crack the cipher";
    /// let cipher_text = CipherText::new("68656c6c6f776f726c64".to_string(), "a-zA-Z0-9.,?! :;\'\"".to_string());
    /// cypher_text.enter_crib_try(&input);
    /// ```
    pub fn new(cipher_text: String, char_set: String) -> Result<Self, regex::Error> {
        let formatted = format!("^[{}]+$", char_set);
        let regex_match_set = Regex::new(&formatted)?;
        Ok(CipherText {
            history: Vec::new(),
            cipher_text,
            regex_match_set,
        })
    }

    /// Enter a crib attempt to try and crack the cipher text. The crib attempt is XOR'd with the cipher text
    /// and the result is printed to the console. If the crib attempt uncovers a possible match, the text is printed
    /// with a *** to indicate a possible match.
    /// # Example
    /// ```
    /// let input = "some phrase to try and crack the cipher";
    /// let cipher_text = CipherText::new("68656c6c6f776f726c64".to_string(), "a-zA-Z0-9.,?! :;\'\"".to_string());
    /// cypher_text.enter_crib_try(&input);
    /// ```
    pub fn enter_crib_try(&mut self, input: &str) {
        self.history.push(input.to_string());
        let last_entry = self.history.iter().last().unwrap();
        let chunks = xor_ascii_with_crib(&self.cipher_text, last_entry);
        print_chunks(chunks, &self.regex_match_set);
    }

    /// Prints the history of crib attempts to the console. If there is no history to display, a message is printed
    /// that there is no history.
    /// # Example
    /// ```
    /// let input = "some phrase to try and crack the cipher";
    /// let cipher_text = CipherText::new("68656c6c6f776f726c64".to_string(), "a-zA-Z0-9.,?! :;\'\"".to_string());
    /// cypher_text.enter_crib_try(&input);
    /// cypher_text.print_history();
    /// ```
    pub fn print_history(&self) {
        if self.history.is_empty() {
            println!("[No history to display]");
            return;
        }
        println!("History of crib attempts:");
        for (index, item) in self.history.iter().enumerate() {
            print!("{}: {}", index, item);
        }
        io::stdout().flush().expect("Failed to flush stdout");
    }
}

fn print_chunks(chunks: Vec<String>, regex_match_set: &Regex) {
    chunks.iter().enumerate().for_each(|(index, chunk)| {
        let decoded = hex_to_ascii(chunk).unwrap();
        let found = regex_match_set.find(&decoded);
        match found {
            Some(_) => {
                println!("{}: {} *** (possible drag match)", index, decoded);
            }
            None => {
                println!("{}: {}", index, decoded);
            }
        }
    });
}

fn xor_ascii_with_crib(cipher_text: &str, crib: &str) -> Vec<String> {
    let crib_in_hex = ascii_to_hex(crib);
    let length = cipher_text.len() - crib_in_hex.len() + 1;

    let mut chunks: Vec<String> = Vec::new();
    for i in (0..=length).step_by(2) {
        let attempt_slice = i + crib_in_hex.len();
        let slice = if attempt_slice > cipher_text.len() {
            &cipher_text[i..]
        } else {
            &cipher_text[i..attempt_slice]
        };
        let xored = xor_strings(slice, &crib_in_hex);
        chunks.push(xored);
    }
    chunks
}

fn ascii_to_hex(s: &str) -> String {
    s.as_bytes().iter().fold(String::new(), |mut acc, byte| {
        acc.push_str(&format!("{:02x}", byte));
        acc
    })
}

fn hex_to_ascii(s: &str) -> Result<String, ParseIntError> {
    let bytes: Result<Vec<u8>, ParseIntError> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();
    let ascii_string = String::from_utf8(bytes?).unwrap();
    Ok(ascii_string)
}

fn xor_strings(a: &str, b: &str) -> String {
    let mut result = String::new();
    let length = a.len().min(b.len());
    for i in 0..length {
        let a_val = u8::from_str_radix(&a[i..i + 1], 16).unwrap();
        let b_val = u8::from_str_radix(&b[i..i + 1], 16).unwrap();
        let xor_val = a_val ^ b_val;
        result.push_str(&format!("{:01x}", xor_val));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_to_hex() {
        let input = "hello";
        let expected = "68656c6c6f";
        let result = ascii_to_hex(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hex_to_ascii() {
        let input = "68656c6c6f";
        let expected = "hello";
        let result = hex_to_ascii(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_xor_strings() {
        let a = "68656c6c6f"; // "hello" in hex
        let b = "776f726c64"; // "world" in hex
        let expected = "1f0a1e000b"; // XOR result in hex
        let result = xor_strings(a, b);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_xor_ascii_with_crib() {
        let cipher_text = "68656c6c6f776f726c64"; // "helloworld" in hex
        let crib = "776f726c64"; // "world" in hex
        let expected = vec![
            "5f525a0a584559115a50".to_string(), // XOR result for "hello" ^ "world"
        ];
        let result = xor_ascii_with_crib(cipher_text, crib);
        assert_eq!(result, expected);
    }
}
