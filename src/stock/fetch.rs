use futures::future::join_all;
use time::OffsetDateTime;
use yahoo_finance_api::YahooConnector;

use super::data::{Fund, StockRow, TRADING_DAYS_PER_MONTH, TRADING_DAYS_PER_YEAR};

/// Fetches and prints funds each with current price and daily/monthly/yearly/portfolio-start change.
/// Skips (and reports) any ticker that fails to fetch.
pub async fn fetch_funds(
    provider: &YahooConnector,
    funds: &[Fund],
    start: OffsetDateTime,
    end: OffsetDateTime,
) -> Vec<StockRow> {
    let mut portfolio: Vec<(StockRow, &Fund)> = Vec::new();
    let mut stocks: Vec<StockRow> = Vec::new();
    let mut tickers = Vec::new();

    for fund in funds {
        tickers.push(fund.ticker);
    }

    let stock_rows = get_stock_rows(provider, &tickers, start, end).await;

    for (i, result) in stock_rows.into_iter().enumerate() {
        match result {
            Some(stock) => {
                portfolio.push((stock.clone(), &funds[i]));
                stocks.push(set_fund_name_in_stock_row(&funds[i], &stock));
            }
            None => println!("Kunde inte hämta data för {}", funds[i].name),
        }
    }

    let (daily, monthly, yearly, total) = calculate_portfolio_change(&portfolio);
    let portfolio_stockrow = StockRow {
        name: "Portfolio".to_string(),
        price: None,
        daily_change: daily,
        monthly_change: monthly,
        yearly_change: yearly,
        start_change: total,
    };
    stocks.push(portfolio_stockrow);

    stocks
}

/// Fetches and prints stocks each with current price and daily/monthly/yearly/portfolio-start change.
/// Skips (and reports) any ticker that fails to fetch.
pub async fn fetch_stocks(
    provider: &YahooConnector,
    tickers: &[&str],
    start: OffsetDateTime,
    end: OffsetDateTime,
) -> Vec<StockRow> {
    let stock_rows = get_stock_rows(provider, tickers, start, end).await;

    let mut stocks: Vec<StockRow> = Vec::new();
    for (i, result) in stock_rows.into_iter().enumerate() {
        match result {
            Some(stock) => stocks.push(stock),
            None => println!("Kunde inte hämta data för {}", tickers[i]),
        }
    }

    stocks
}

/// Convert tickets into StockRows
/// Creates all futures
/// Fetches all futures at the same time - keeps the inital order
async fn get_stock_rows(
    provider: &YahooConnector,
    tickers: &[&str],
    start: OffsetDateTime,
    end: OffsetDateTime,
) -> Vec<Option<StockRow>> {
    let mut futures = Vec::new();
    for ticker in tickers {
        futures.push(get_data(provider, ticker, start, end));
    }

    join_all(futures).await
}

/// Fetches one year of daily quotes for `ticker` and computes price
/// changes relative to yesterday, ~1 month ago, ~1 year ago and ~start of portfolio.
/// Returns `None` if the fetch fails or fewer than 2 data points exist.
async fn get_data(
    provider: &YahooConnector,
    ticker: &str,
    start: OffsetDateTime,
    end: OffsetDateTime,
) -> Option<StockRow> {
    let data = provider
        .get_quote_history(ticker, start, end)
        .await
        .ok()?
        .quotes()
        .ok()?;

    if data.len() < 2 {
        return None;
    }

    let last: f64 = data.last()?.close;
    let day_before: f64 = data[data.len() - 2].close;
    let month_ago: f64 = data[data.len().saturating_sub(TRADING_DAYS_PER_MONTH)].close;
    let year_ago: f64 = data[data.len().saturating_sub(TRADING_DAYS_PER_YEAR)].close; //252 
    let start_ago: f64 = data.first()?.close;

    Some(StockRow {
        name: ticker.to_string(),
        price: Some(last),
        daily_change: percentage(day_before, last),
        monthly_change: percentage(month_ago, last),
        yearly_change: percentage(year_ago, last),
        start_change: percentage(start_ago, last),
    })
}

/// Creates a new StockRow with a new name, mainly for tickers without a good name
fn set_fund_name_in_stock_row(fund: &Fund, stock: &StockRow) -> StockRow {
    StockRow {
        name: fund.name.to_string(),
        ..*stock
    }
}

/// Percentage change from `start` to `end`, relative to `start`.
fn percentage(start: f64, end: f64) -> f64 {
    (end - start) / start * 100.0
}

/// Calculates total change by fund weigth in a portfolio
fn calculate_portfolio_change(stocks: &[(StockRow, &Fund)]) -> (f64, f64, f64, f64) {
    let mut daily = 0.0;
    let mut monthly = 0.0;
    let mut yearly = 0.0;
    let mut inverst_total = 0.0;

    for (stock, fund) in stocks {
        daily += stock.daily_change * fund.weigth;
        monthly += stock.monthly_change * fund.weigth;
        yearly += stock.yearly_change * fund.weigth;
        inverst_total += stock.start_change * fund.weigth;
    }

    (daily, monthly, yearly, inverst_total)
}
