use std::io::{self, Write};

use crate::pages::is_valid_page_number;

pub enum UserInput {
    Page(u32),
    Quit,
    Invalid,
    Stock,
}

/// Reads a line and converts it to UserInput
///
/// - "q" / "quit" / Ctrl+D → Quit
/// - "stock" → Stock
/// - a valid page number → Page(n)
/// - anything else → Invalid
pub fn get_input() -> UserInput {
    if io::stdout().flush().is_err() {
        return UserInput::Invalid;
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => return UserInput::Quit, // EOF (Ctrl+D) — read_line returns Ok(0) with nothing read
        Ok(_) => {}                      // got input, continue below
        Err(_) => return UserInput::Invalid,
    }

    let trimmed = input.trim().to_ascii_lowercase();

    match trimmed.as_str() {
        "q" | "quit" => return UserInput::Quit,
        "stock" => return UserInput::Stock,
        _ => {}
    }

    match is_valid_page_number(&trimmed) {
        Some(n) => UserInput::Page(n),
        None => UserInput::Invalid,
    }
}
