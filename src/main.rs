use std::collections::HashSet;
mod stock;
use stock::stock;

mod input;
use input::get_input;

mod pages; 
use pages::{page_exist, get_page, add_page_nbrs, page_has_changed};

use crate::input::UserInput;

static START_URL_PAGE_NBR: u32 = 101;
static INPUT_MESSAGE: &str = " Starsida: 101, Stänga: q eller quit, Börs: stock\n Sida eller börs: ";
static WRONG_INPUT: &str = "Fel input - enbart tal, ex 106";

// tokio::main är macro executor - rust vet inte hur async-funktioner körs utan dem
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut page_nbrs: HashSet<u32> = HashSet::new();
    let mut starting_page: String = get_page(START_URL_PAGE_NBR).await?;
    println!("{}",starting_page);
    print_info();

    add_page_nbrs(&mut page_nbrs, &starting_page);

    loop {
        let new_staring_page: String = get_page(START_URL_PAGE_NBR).await?;
        if page_has_changed(&starting_page, &new_staring_page) {
            starting_page = new_staring_page;
            add_page_nbrs(&mut page_nbrs, &starting_page);
        }

        match get_input() {
            UserInput:: Quit => break,
            UserInput::Invalid => println!("{}", WRONG_INPUT),
            UserInput::Stock => {
                stock().await;
                print_info();
            },

            UserInput::Page(page_nbr) => {
                if page_exist(&page_nbrs, page_nbr) {
                let page:String = get_page(page_nbr).await?;
                println!("{}",page);
                add_page_nbrs(&mut page_nbrs, &page);
                print_info();
                }
            
            }
        
        }
    }

    Ok(())
}

fn print_info(){
    println!("");
    println!("{}",INPUT_MESSAGE);
}