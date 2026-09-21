use reqwest::Response;
use std::collections::HashSet;

const MIN_PAGE_NUMBER: u32 = 100;
const MAX_PAGE_NUMBER: u32 = 899;

/// Returns true if the given page number exists in the discovered-pages set.
pub fn page_exist(page_nbrs: &HashSet<u32>, page_nbr: u32) -> bool {
    page_nbrs.contains(&page_nbr)
}
/// Fetches the Text-TV page with the given page number and returns its text content.
pub async fn get_page(page_nbr: u32) -> Result<String, Box<dyn std::error::Error>> {
    let url: String = format!(
        "https://api.texttv.nu/api/get/{}?includePlainTextContent=1&app=LTHtexttv",
        page_nbr
    );
    let resp: Response = reqwest::get(url).await?;
    let text: String = resp.text().await?;
    let json: serde_json::Value = serde_json::from_str(&text)?;
    Ok(json[0]["content_plain"][0]
        .as_str()
        .unwrap_or("(ingen text hittad)")
        .to_string())
}

/// Scans `content` for page numbers and ranges, adding any found to `page_nbrs`.
pub fn add_page_nbrs(page_nbrs: &mut HashSet<u32>, content: &str) {
    for line in content.lines() {
        for word in line.split_whitespace() {
            if let Some((start, end)) = parse_range(word) {
                for n in start..=end {
                    page_nbrs.insert(n);
                }
            } else if let Some(n) = is_valid_page_number(word) {
                page_nbrs.insert(n);
            }
        }
    }
}

/// Returns `Some(n)` if `word` contains a valid page number within the allowed range, else `None`.
pub fn is_valid_page_number(word: &str) -> Option<u32> {
    let digits = get_digits(word);

    if digits.is_empty() {
        return None;
    }

    let n: u32 = digits.parse().ok()?;
    if (MIN_PAGE_NUMBER..=MAX_PAGE_NUMBER).contains(&n) {
        Some(n)
    } else {
        None
    }
}
/// Strips trailing non-digit characters, then returns the trailing run of digits in `word`.
fn get_digits(word: &str) -> &str {
    let trimmed: &str = word.trim_end_matches(|c: char| !c.is_ascii_digit());

    let start: usize = trimmed
        .rfind(|c: char| !c.is_ascii_digit())
        .map(|i| i + 1)
        .unwrap_or(0);

    &trimmed[start..]
}

/// Parses a hyphenated range like `"405f-412f"` into `Some((405, 412))`, else `None`.
fn parse_range(word: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = word.split('-').collect();

    if parts.len() != 2 {
        return None;
    }

    let start = get_digits(parts[0]).parse().ok()?;
    let end = get_digits(parts[1]).parse().ok()?;

    Some((start, end))
}

/// Returns true if `new_content` differs from `old_content`.
pub fn page_has_changed(old_content: &str, new_content: &str) -> bool {
    old_content != new_content
}
