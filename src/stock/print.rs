use time::{Duration, OffsetDateTime, format_description};
use yahoo_finance_api::{self as yahoo};

use super::fetch::{fetch_funds, fetch_stocks};
use crate::stock::data::{
    BOLD, GLOBAL_FUNDS, GLOBAL_START, LYSA_FUNDS, LYSA_START, OMX30_TICKERS, RESET, StockRow,
};

/// Prints the full report: Global funds, Lysa funds, and OMX30 stocks,
/// each with a header and a row per holding (plus a portfolio summary row
/// for the two fund sections).
pub async fn print_stocks_and_funds() {
    let provider = yahoo::YahooConnector::new().unwrap();
    let now = OffsetDateTime::now_utc();
    let five_years_ago = now - Duration::days(5 * 365 + 1);
    let global_start = *GLOBAL_START;
    let lysa_start = *LYSA_START;

    let (omx30, lysa, global) = tokio::join!(
        fetch_stocks(&provider, OMX30_TICKERS, five_years_ago, now),
        fetch_funds(&provider, LYSA_FUNDS, lysa_start, now),
        fetch_funds(&provider, GLOBAL_FUNDS, global_start, now)
    );

    print_header("Global", global_start);
    print_stock(&global);

    print_header("Lysa", lysa_start);
    print_stock(&lysa);

    print_header("OMX30", five_years_ago);
    print_stock(&omx30);
}

/// Prints the table title and a bold column header row.
pub fn print_header(title: &str, date: OffsetDateTime) {
    println!();
    println!("{:=^90}", title);
    println!();
    println!(
        "{}{:<25} {:>8} {:>8} {:>8} {:>8} {:>8} {}",
        BOLD,
        "Aktie",
        "Pris",
        "Dagens",
        "Månadens",
        "Årets",
        format!("Sen {}", format_date(date)),
        RESET
    );
    println!("{}", "─".repeat(100));
}

pub fn format_date(date: OffsetDateTime) -> String {
    let format = format_description::parse_borrowed::<2>("[year]-[month]-[day]").unwrap();
    date.format(&format).unwrap()
}

/// Prints one stock's row, with each percentage change colorized
/// green (positive) or red (negative).
pub fn print_stock(stocks: &Vec<StockRow>) {
    for stock in stocks {
        let price_str = match stock.price {
            Some(p) => format!("{:>8.2}", p),
            None => format!("{:>8}", "-"),
        };
        println!(
            "{:<25} {:>8} {:>8} {:>8} {:>8} {:>8}",
            stock.name,
            price_str,
            colorize(stock.daily_change),
            colorize(stock.monthly_change),
            colorize(stock.yearly_change),
            colorize(stock.start_change)
        );
    }
}

/// Formats a percentage value with a forced sign and ANSI color
/// (green for >= 0, red for negative)
pub fn colorize(value: f64) -> String {
    let plain = format!("{:+.1}%", value);
    let padded = format!("{:>8}", plain);
    let color = if value >= 0.0 { "\x1b[32m" } else { "\x1b[31m" };
    format!("{}{}\x1b[0m", color, padded)
}
