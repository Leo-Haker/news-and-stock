use reqwest::Response;

// tokio::main är macro executor - rust vet inte hur async-funktioner körs utan dem
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Gör ett GET-anrop till texttv-API:et för sida 101
    let resp = reqwest::get("https://api.texttv.nu/api/get/101?includePlainTextContent=1&app=LTHtexttv").await?;

    // 2. Läs svaret som text
    let text = resp.text().await?;

    // Parsa den råa JSON-strängen till en `serde_json::Value` — en generisk
// representation som kan vara ett objekt, en array, en sträng, etc.
// Vi vet ännu inte den exakta strukturen i Rust-typer, så Value funkar
// som en "dynamisk" JSON-struktur vi kan indexera in i.
let json: serde_json::Value = serde_json::from_str(&text)?;

// API-svaret är en ARRAY med ETT sidobjekt: json[0] = det objektet.
// "content_plain" är ett fält i objektet som själv är en array med
// EN sträng (hela sidans text): json[0]["content_plain"][0].
println!("{}", json[0]["content_plain"][0]);

//hämtar ut rubriker och sidnummer - se i terminalen
let content = json[0]["content_plain"][0]
    .as_str()                        // Option<&str> — None om det inte var en sträng
    .unwrap_or("(ingen text hittad)"); // fallback om något gick fel

println!("{}", content);


let content2 = get_page(106).await?;
    println!("{}", content2);

if let Some(obj) = json[0].as_object(){
    for key in obj.keys() {
        println!("{}",key);
    }
}


    Ok(())
}

async fn get_page(page_nbr: u32) -> Result<String, Box<dyn std::error::Error>> {
    let url: String = format!("https://api.texttv.nu/api/get/{}?includePlainTextContent=1&app=LTHtexttv", page_nbr);
    let resp:Response = reqwest::get(url).await?;
    let text : String = resp.text().await?;
    let json: serde_json::Value = serde_json::from_str(&text)?;
    Ok(json[0]["content_plain"][0].as_str().unwrap_or("(ingen text hittad)").to_string())
}