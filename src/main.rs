use std::collections::HashSet;

mod input;
use input::get_input;

mod pages; 
use pages::{page_exist, get_page, add_page_nbrs, page_has_changed};

use crate::input::UserInput;

static START_URL_PAGE_NBR: u32 = 101;
static WRONG_INPUT: &str = "Fel input - enbart tal, ex 106";

// tokio::main är macro executor - rust vet inte hur async-funktioner körs utan dem
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut page_nbrs: HashSet<u32> = HashSet::new();
    let mut starting_page: String = get_page(START_URL_PAGE_NBR).await?;
    println!("{}",starting_page);

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
            UserInput::Page(page_nbr) => {
                if page_exist(&page_nbrs, page_nbr) {
                let page:String = get_page(page_nbr).await?;
                println!("{}",page);
                add_page_nbrs(&mut page_nbrs, &page);
                }
            
            }
        
        }
    }

    Ok(())
}







//     // 1. Gör ett GET-anrop till texttv-API:et för sida 101
//     let resp = reqwest::get("https://api.texttv.nu/api/get/101?includePlainTextContent=1&app=LTHtexttv").await?;

//     // 2. Läs svaret som text
//     let text = resp.text().await?;

//     // Parsa den råa JSON-strängen till en `serde_json::Value` — en generisk
// // representation som kan vara ett objekt, en array, en sträng, etc.
// // Vi vet ännu inte den exakta strukturen i Rust-typer, så Value funkar
// // som en "dynamisk" JSON-struktur vi kan indexera in i.
// let json: serde_json::Value = serde_json::from_str(&text)?;

// // API-svaret är en ARRAY med ETT sidobjekt: json[0] = det objektet.
// // "content_plain" är ett fält i objektet som själv är en array med
// // EN sträng (hela sidans text): json[0]["content_plain"][0].
// println!("{}", json[0]["content_plain"][0]);

// //hämtar ut rubriker och sidnummer - se i terminalen
// let content = json[0]["content_plain"][0]
//     .as_str()                        // Option<&str> — None om det inte var en sträng
//     .unwrap_or("(ingen text hittad)"); // fallback om något gick fel

// println!("{}", content);


// let content2 = get_page(106).await?;
//     println!("{}", content2);

// if let Some(obj) = json[0].as_object(){
//     for key in obj.keys() {
//         println!("{}",key);
//     }
// }

// //All page numbers









// add_page_nbrs(&mut page_nbrs, content);

// print!("Numbers in HashSet");
// print!("{:?}",page_nbrs);









