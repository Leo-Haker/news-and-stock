mod input;
mod pages;
mod stock;

use input::{UserInput, get_input};
use pages::{add_page_nbrs, get_page, page_exist, page_has_changed};
use stock::print_stocks_and_funds;

use std::collections::HashSet;

const START_URL_PAGE_NBR: u32 = 100;
const INPUT_MESSAGE: &str = " Starsida: 100, Stänga: q eller quit, Börs: stock\n Sida eller börs: ";
const WRONG_INPUT: &str = "Fel input - enbart tal, ex 106";

// Rust has no built-in way to run async code, just the syntax.
// #[tokio::main] sets up a runtime for us so main() can actually be async.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch and print the front page once before the loop starts.
    let mut page_nbrs: HashSet<u32> = HashSet::new();
    let mut starting_page: String = get_page(START_URL_PAGE_NBR).await?;
    println!("{}", starting_page);
    print_info();
    add_page_nbrs(&mut page_nbrs, &starting_page);

    loop {
        // Check whether the front page has changed since last time (SVT
        // updates it continuously), and add any new page numbers it contains.
        let new_starting_page: String = get_page(START_URL_PAGE_NBR).await?;
        if page_has_changed(&starting_page, &new_starting_page) {
            starting_page = new_starting_page;
            add_page_nbrs(&mut page_nbrs, &starting_page);
        }

        match get_input() {
            UserInput::Quit => break,
            UserInput::Invalid => println!("{}", WRONG_INPUT),
            UserInput::Stock => {
                print_stocks_and_funds().await;
                print_info();
            }

            UserInput::Page(page_nbr) => {
                if page_exist(&page_nbrs, page_nbr) {
                    let page: String = get_page(page_nbr).await?;
                    println!("{}", page);
                    add_page_nbrs(&mut page_nbrs, &page);
                    print_info();
                } else {
                    println!("Sidan finns inte i listan just nu.");
                }
            }
        }
    }

    Ok(())
}

/// Prints a blank line followed by the input prompt.
fn print_info() {
    println!();
    println!("{}", INPUT_MESSAGE);
}

