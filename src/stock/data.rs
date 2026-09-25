use once_cell::sync::Lazy;
use time::{Date, Month, OffsetDateTime};

// ANSI escape codes for terminal text styling.
pub const BOLD: &str = "\x1b[1m";
pub const RESET: &str = "\x1b[0m"; // always pair with a styled string, or formatting "leaks" to the rest of the terminal

pub const TRADING_DAYS_PER_MONTH: usize = 21;
pub const TRADING_DAYS_PER_YEAR: usize = 252;

/// A single row of stock data ready to be printed: current price and
/// percentage change over three time horizons.
#[derive(Clone)]
pub struct StockRow {
    pub name: String,
    pub price: Option<f64>,
    pub daily_change: f64,
    pub monthly_change: f64,
    pub yearly_change: f64,
    pub start_change: f64,
}

pub struct Fund {
    pub ticker: &'static str,
    pub name: &'static str,
    pub weigth: f64,
}

///Yahoo Finance tickers for Global coverage
pub const GLOBAL_FUNDS: &[Fund] = &[
    Fund {
        ticker: "0P0001Q6FC.ST",
        name: "DNB Global Indeks S",
        weigth: 0.85,
    },
    Fund {
        ticker: "0P0001H4TL.ST",
        name: "Avanza Emergin Markets",
        weigth: 0.15,
    },
];

pub static GLOBAL_START: Lazy<OffsetDateTime> = Lazy::new(|| {
    Date::from_calendar_date(2025, Month::August, 25)
        .unwrap()
        .midnight()
        .assume_utc()
});

///Yahoo Finance tickers for Lysa Global
pub const LYSA_FUNDS: &[Fund] = &[
    Fund {
        ticker: "0P00019MOJ.ST",
        name: "Lysa Global", //Lysa Global Equity Broad C
        weigth: 0.7669,
    },
    Fund {
        ticker: "0P0001UE4H.ST",
        name: "Lysa Emerging Markets", //Lysa Emerging Markets Equity Broad B
        weigth: 0.1290,
    },
    Fund {
        ticker: "0P0001UE4I.ST",
        name: "Lysa Global Small Cap", //Lysa Global Small Cap Equity Broad B
        weigth: 0.1041,
    },
];

pub static LYSA_START: Lazy<OffsetDateTime> = Lazy::new(|| {
    Date::from_calendar_date(2022, Month::June, 1)
        .unwrap()
        .midnight()
        .assume_utc()
});

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
