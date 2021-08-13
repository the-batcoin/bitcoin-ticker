use super::digit::Digit;
use super::digit::Msg as DigitMsg;
use js_sys::TypeError;
use sauron::prelude::*;
use sauron::wasm_bindgen::JsCast;
use serde::Deserialize;
use web_sys::Response;

const API_URL: &'static str = "https://api.coindesk.com/v1/bpi/currentprice.json";
const DIGIT_RANGE_END: u32 = 8;
const DIGIT_RANGE: std::ops::Range<u32> = 0..DIGIT_RANGE_END;

#[derive(Deserialize, Debug, PartialEq, Clone, Default)]
pub struct BPIUSD {
    pub rate: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Default)]
pub struct BPI {
    pub USD: BPIUSD,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Default)]
pub struct TimeData {
    pub updated: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Default)]
pub struct RequestData {
    pub time: TimeData,
    pub bpi: BPI,
}

#[derive(Debug)]
pub enum Msg {
    ReceivedData(Result<RequestData, Response>),
    RequestError(TypeError),
    DigitMsg(DigitMsg),
    IntervalExpired,
}

pub struct Ticker {
    price: f64,
    date_time: String,
    digits: Vec<Digit>,
}

impl Ticker {
    pub fn create() -> Self {
        Self {
            price: 100000.,
            date_time: String::from("June 5rd, 2077"),
            digits: (DIGIT_RANGE).map(|_| Digit::create()).collect(),
        }
    }

    pub fn fetch_data() -> Cmd<Self, Msg> {
        Http::fetch_with_text_response_decoder(
            &API_URL,
            |v: String| {
                let data: Result<RequestData, _> = serde_json::from_str(&v);
                trace!("data: {:#?}", data);
                data.expect("Error deserializing data")
            },
            Msg::ReceivedData,
            Msg::RequestError,
        )
    }

    pub fn minute_interval() -> Cmd<Self, Msg> {
        Cmd::new(move |program| {
            let clock: Closure<dyn Fn()> = Closure::wrap(Box::new(move || {
                program.dispatch(Msg::IntervalExpired);
            }));

            web_sys::window()
                .expect("no global `window` exists")
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    clock.as_ref().unchecked_ref(),
                    1000 * 60,
                )
                .expect("Unable to start interval");
            clock.forget();
        })
    }

    pub fn price_str_extract_digit(price: &str, position: usize) -> u32 {
        let digit_str = price.get(position..position + 1);
        let digit_str = match digit_str {
            Some(d) => d,
            None => {
                error!("couldn't get digit str at position {}", position);
                "0"
            }
        };
        let digit_num: u32 = match digit_str.parse() {
            Ok(d) => d,
            Err(e) => {
                error!("{:?}", e);
                0
            }
        };
        digit_num
    }

    fn set_price_digits(&mut self, price: &str) {
        for digit_pos in DIGIT_RANGE {
            let digit_pos = digit_pos as usize;
            let digit_num = Ticker::price_str_extract_digit(price, digit_pos);
            self.digits[digit_pos].update(DigitMsg::SelectDigit(digit_num as i32));
        }
    }
}

impl Component<Msg> for Ticker {
    fn init(&self) -> Cmd<Self, Msg> {
        Cmd::batch(vec![Ticker::fetch_data(), Ticker::minute_interval()])
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        debug!("Updating Ticker with msg {:?}", msg);
        match msg {
            Msg::IntervalExpired => {
                // Http request
                Ticker::fetch_data()
            }
            Msg::ReceivedData(result) => {
                match result {
                    Ok(data) => {
                        self.price = data.bpi.USD.rate.replace(",", "").parse().unwrap();
                        self.date_time = data.time.updated;
                        let formatted_price = format!("{:0>8}", self.price.round() as u32);
                        info!("{}", formatted_price);
                        self.set_price_digits(&formatted_price);
                    }
                    Err(e) => error!("{:?}", e),
                }
                Cmd::none()
            }
            _ => Cmd::none(),
        }
    }

    fn view(&self) -> Node<Msg> {
        web_sys::console::debug(&js_sys::Array::from(&JsValue::from_str("Drawing Ticker")));
        node! {
            <div style="display: block">
            <p>{text(self.date_time.clone())}</p>
            <div class="ticker">
                <div class="dollar-sign">"$"</div>
                { for digit in self.digits.iter() {
                                                      digit.view().map_msg(Msg::DigitMsg)
                                                  }
                }
            </div>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_digit_from_formatted_price() {
        let formatted_price = "00000046444";
        let extracted = Ticker::price_str_extract_digit(formatted_price, 7);
        assert_eq!(6, extracted);
    }

    #[test]
    fn str_get() {
        let formatted_price = "00000046444";
        assert_eq!("6", &formatted_price[7..8]);
    }
}
