use yahoo_finance_api::{self as yahoo, YahooConnector};

/// A single row of stock data ready to be printed: current price and
/// percentage change over three time horizons.
struct StockRow {
    name: String,
    price: f64,
    daily_change: f64,
    monthly_change: f64,
    yearly_change: f64,
}

// ANSI escape codes for terminal text styling.
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m"; // always pair with a styled string, or formatting "leaks" to the rest of the terminal

/// Yahoo Finance tickers for all 30 constituents of the OMXS30 index.
/// Verified against Nordnet's list (as of 2026-09-03):
/// https://www.nordnet.se/aktier/inspiration/listor/omx-stockholm-30
pub const OMX30_TICKERS: &[&str] = &[
    "ABB.ST",
    "ADDT-B.ST",
    "ALFA.ST",
    "ASSA-B.ST",
    "AZN.ST",
    "ATCO-A.ST",
    "BOL.ST",
    "EPI-A.ST",
    "EQT.ST",
    "ERIC-B.ST",
    "ESSITY-B.ST",
    "EVO.ST",
    "HM-B.ST",
    "HEXA-B.ST",
    "INDU-C.ST",
    "INVE-B.ST",
    "LIFCO-B.ST",
    "NIBE-B.ST",
    "NDA-SE.ST",
    "SAAB-B.ST",
    "SAND.ST",
    "SEB-A.ST",
    "SKA-B.ST",
    "SKF-B.ST",
    "SCA-B.ST",
    "SHB-A.ST",
    "SWED-A.ST",
    "TEL2-B.ST",
    "TELIA.ST",
    "VOLV-B.ST",
];

/// Fetches and prints a table of all OMX30 stocks with current price and
/// daily/monthly/yearly percentage change. Skips (and reports) any ticker
/// that fails to fetch, rather than aborting the whole table.
pub async fn stock() {
    let provider = yahoo::YahooConnector::new().unwrap();

    print_header();

    for ticker in OMX30_TICKERS {
        let stock = get_stock_info(&provider, ticker).await;
        if let Some(stock) = stock {
            print_stock(stock);
        } else {
            println!("Kunde inte hämta data för {}", ticker)
        }
    }
}

/// Fetches one year of daily quotes for `ticker` and computes price
/// changes relative to yesterday, ~1 month ago, and ~1 year ago.
/// Returns `None` if the fetch fails or fewer than 2 data points exist.
async fn get_stock_info(provider: &YahooConnector, ticker: &str) -> Option<StockRow> {
    let year_data = provider
        .get_quote_range(ticker, "1d", "1y")
        .await
        .ok()?
        .quotes()
        .ok()?;

    if year_data.len() < 2 {
        return None;
    }

    let last: f64 = year_data.last()?.close;
    let day_before: f64 = year_data[year_data.len() - 2].close;
    let month_ago: f64 = year_data[year_data.len().saturating_sub(21)].close;
    let year_ago: f64 = year_data.first()?.close;

    Some(StockRow {
        name: ticker.to_string(),
        price: last,
        daily_change: percentage(day_before, last),
        monthly_change: percentage(month_ago, last),
        yearly_change: percentage(year_ago, last),
    })
}

/// Prints the table title and a bold column header row.
fn print_header() {
    println!("{:=^90}", " OMX30 ");
    println!(
        "{}{:<12} {:>8} {:>8} {:>8} {:>8}{}",
        BOLD, "Aktie", "Pris", "Dagens", "Månadens", "Årets", RESET
    );
    println!("{}", "─".repeat(50));
}

/// Prints one stock's row, with each percentage change colorized
/// green (positive) or red (negative).
fn print_stock(
    StockRow {
        name,
        price,
        daily_change,
        monthly_change,
        yearly_change,
    }: StockRow,
) {
    println!(
        "{:<12} {:>8.2} {} {} {}",
        name,
        price,
        colorize(daily_change),
        colorize(monthly_change),
        colorize(yearly_change)
    );
}

/// Formats a percentage value with a forced sign and ANSI color
/// (green for >= 0, red for negative)
fn colorize(value: f64) -> String {
    let plain = format!("{:+.1}%", value);      
    let padded = format!("{:>8}", plain);          
    let color = if value >= 0.0 { "\x1b[32m" } else { "\x1b[31m" };
    format!("{}{}\x1b[0m", color, padded)
}

/// Percentage change from `start` to `end`, relative to `start`.
fn percentage(start: f64, end: f64) -> f64 {
    (end - start) / start * 100.0
}
