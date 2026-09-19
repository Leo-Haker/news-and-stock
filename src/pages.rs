use std::collections::HashSet;
use reqwest::Response;

const MIN_PAGE_NUMBER: u32 = 100;
const MAX_PAGE_NUMBER: u32 = 899;


pub fn page_exist(page_nbrs: &HashSet<u32>, page_nbr: u32) -> bool {
    return page_nbrs.contains(&page_nbr);
}


pub async fn get_page(page_nbr: u32) -> Result<String, Box<dyn std::error::Error>> {
    let url: String = format!("https://api.texttv.nu/api/get/{}?includePlainTextContent=1&app=LTHtexttv", page_nbr);
    let resp:Response = reqwest::get(url).await?;
    let text : String = resp.text().await?;
    let json: serde_json::Value = serde_json::from_str(&text)?;
    Ok(json[0]["content_plain"][0].as_str().unwrap_or("(ingen text hittad)").to_string())
}

pub fn add_page_nbrs(page_nbrs: &mut HashSet<u32>, content: &str) {
    for line in content.lines() {
        for word in line.split_whitespace() {
            if let Some((start, end)) = parse_range(word) {
                for n in start..= end {
                    page_nbrs.insert(n);
                }
            } else if let Some(n) = is_valid_page_number(word) {
                page_nbrs.insert(n);
            }
        }
    }
}

pub fn is_valid_page_number(word: &str) -> Option<u32> {
    //
    let digits = get_digits(word);

    if digits.is_empty() {
        return None;
    }

    let n: u32 = digits.parse().ok()?;
    if n >= MIN_PAGE_NUMBER && n <= MAX_PAGE_NUMBER {
        Some(n)
    } else {
        None
    }
}
//@return &str
//Removes NaN from the end.
//Then checks from which index the number starts in the word
fn get_digits (word: &str) -> &str {
    let trimmed: &str = word.trim_end_matches(|c: char| !c.is_ascii_digit());

    let start:usize  =trimmed
                            .rfind(|c: char| !c.is_ascii_digit())
                            .map(|i| i + 1)
                            .unwrap_or(0);

    &trimmed[start..]
    

} 

//@return Option<(u32, u32)>
//If word is in a valid interval "X-Y", return (X, Y). Else None
fn parse_range(word: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = word.split('-').collect();

    if parts.len() != 2 {
        return None;
    }

    let start = get_digits(parts[0]).parse().ok()?;
    let end = get_digits(parts[1]).parse().ok()?;

    Some((start,end))
}

pub fn page_has_changed(old_content: &str, new_content: &str) -> bool {
    old_content != new_content
}

