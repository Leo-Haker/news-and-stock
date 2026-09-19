use std::io::{self, Write};


use crate::pages::is_valid_page_number;

static INPUT_MESSAGE: &str = " Starsida: 101, Stänga: q eller quit \n  Ange sidnummer: ";


pub enum UserInput {
    Page(u32),
    Quit,
    Invalid,
}

//@return UserInput
//If Crtl + D, "q" or "quit" return Quit
//Controlls if input correct, returns Page(nbr) or Invalid
pub fn get_input() -> UserInput {
    println!("");
    println!("{}",INPUT_MESSAGE);

    if io::stdout().flush().is_err() {
        return UserInput::Invalid;
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => return UserInput::Quit,
        Ok(_) => {}                         //just normal function - stdin().read_line
        Err(_) => return UserInput::Invalid,
    }

    let trimmed = input.trim().to_ascii_lowercase();

    if trimmed.eq("q") || trimmed.eq("quit") {
        return UserInput::Quit
    }

    match is_valid_page_number(&trimmed) {
        Some(n) => UserInput::Page(n),
        None => UserInput::Invalid
    }


}

