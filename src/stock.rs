// https://www.nordnet.se/aktier/inspiration/listor/omx-stockholm-30
// https://crates.io/crates/yahoo_finance_api

use yahoo_finance_api::{self as yahoo, YahooConnector};

struct StockRow {
    name: String,
    price: f64,
    daily_change: f64,
    monthly_change: f64,
    yearly_change: f64,
}

pub const OMX30_TICKERS: &[&str] = &[
    "ABB.ST", "ADDT-B.ST", "ALFA.ST", "ASSA-B.ST", "AZN.ST", "ATCO-A.ST", "BOL.ST", "EPI-A.ST",
    "EQT.ST", "ERIC-B.ST", "ESSITY-B.ST", "EVO.ST", "HM-B.ST", "HEXA-B.ST", "INDU-C.ST",
    "INVE-B.ST", "LIFCO-B.ST", "NIBE-B.ST", "NDA-SE.ST", "SAAB-B.ST", "SAND.ST", "SEB-A.ST",
    "SKA-B.ST", "SKF-B.ST", "SCA-B.ST", "SHB-A.ST", "SWED-A.ST", "TEL2-B.ST", "TELIA.ST",
    "VOLV-B.ST",
];


pub async fn stock () {
    let provider = yahoo::YahooConnector::new().unwrap();

    print_header();

    for ticker in OMX30_TICKERS {
        let stock = get_stock_info(&provider, ticker).await;
        if let Some(stock) = stock {
          print_stock(stock); 

       } else {
            print!("Kundet inte hämta data för {}", ticker)
       }

    }
  
}

async fn get_stock_info(provider: &YahooConnector,ticker: &str) -> Option<StockRow> {
    let year_data = provider.get_quote_range(ticker, "1d", "1y").await.ok()?.quotes().ok()?;
    
    if year_data.len() < 2 {
        return None;
    }

    let last:f64 = year_data.last()?.close;
    let day_before:f64 = year_data[year_data.len() - 2].close;
    let month_ago:f64 = year_data[year_data.len().saturating_sub(21)].close;
    let year_ago:f64 = year_data.first()?.close;

    Some (StockRow {
        name: ticker.to_string(),
        price: last,
        daily_change: percentage(day_before, last),
        monthly_change: percentage(month_ago, last),
        yearly_change: percentage(year_ago, last)
    })
}


const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

fn print_header() {
    println!("{:=^90}", " OMX30 ");
    println!("{}{:<10} {:>8} {:>8} {:>8} {:>8}{}", BOLD, "Aktie", "Pris", "Dagens", "Månadens", "Årets", RESET);
    println!("{}",  "─".repeat(50));
}

fn print_stock(StockRow { name, price, daily_change, monthly_change, yearly_change }: StockRow) {
    println!("{:<12} {:>8.2} {} {} {}", name, price, colorize(daily_change), colorize(monthly_change), colorize(yearly_change));
}

fn colorize(value: f64) -> String {
    let color = if value >= 0.0 { "\x1b[32m" } else { "\x1b[31m" };
    format!("{}{:+.1}%\x1b[0m", color, value)
}

fn percentage (start: f64, end: f64) -> f64 {
    (end - start) /start * 100.0
}