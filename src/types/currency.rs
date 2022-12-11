use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Currency {
    USD,
    GBP,
    EUR,
    CAD,
    AUD,
    JPY,
    CHF,
    MXN,
    BRL,
    NZD,
    SEK,
    ZAR,
    Unspecified,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// TODO: I wrote these for a previous implementation. Remove if never needed.

// use Currency::*;

// impl From<&str> for Currency {
//     fn from(s: &str) -> Self {
//         match s {
//             "USD" => USD,
//             "GBP" => GBP,
//             "EUR" => EUR,
//             "CAD" => CAD,
//             "AUD" => AUD,
//             "JPY" => JPY,
//             "CHF" => CHF,
//             "MXN" => MXN,
//             "BRL" => BRL,
//             "NZD" => NZD,
//             "SEK" => SEK,
//             "ZAR" => ZAR,
//             _ => Unspecified,
//         }
//     }
// }

// impl From<Currency> for &str {
//     fn from(c: Currency) -> Self {
//         match c {
//             USD => "USD",
//             GBP => "GBP",
//             EUR => "EUR",
//             CAD => "CAD",
//             AUD => "AUD",
//             JPY => "JPY",
//             CHF => "CHF",
//             MXN => "MXN",
//             BRL => "BRL",
//             NZD => "NZD",
//             SEK => "SEK",
//             ZAR => "ZAR",
//             Unspecified => "",
//         }
//     }
// }
