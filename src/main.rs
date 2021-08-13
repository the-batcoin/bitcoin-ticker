use sauron::prelude::*;

#[macro_use]
extern crate log;

mod ticker;
mod digit;

use log::Level;

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(ticker::Ticker::create());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coindesk_json() {
        let test_json = r#"{"time":{"updated":"Aug 11, 2021 22:21:00 UTC","updatedISO":"2021-08-11T22:21:00+00:00","updateduk":"Aug 11, 2021 at 23:21 BST"},"disclaimer":"This data was produced from the CoinDesk Bitcoin Price Index (USD). Non-USD currency data converted using hourly conversion rate from openexchangerates.org","chartName":"Bitcoin","bpi":{"USD":{"code":"USD","symbol":"&#36;","rate":"45,983.3647","description":"United States Dollar","rate_float":45983.3647},"GBP":{"code":"GBP","symbol":"&pound;","rate":"33,151.4768","description":"British Pound Sterling","rate_float":33151.4768},"EUR":{"code":"EUR","symbol":"&euro;","rate":"39,163.7558","description":"Euro","rate_float":39163.7558}}}
"#;
        let deserialized: ticker::RequestData = serde_json::from_str(test_json).unwrap();
        assert_eq!(String::from("45,983.3647"), deserialized.bpi.USD.rate);
        assert_eq!(
            String::from("Aug 11, 2021 22:21:00 UTC"),
            deserialized.time.updated
        );
    }
}
