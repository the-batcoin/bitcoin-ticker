use sauron::prelude::*;
    #[derive(Debug)]
    pub enum Msg {
        SelectDigit(i32),
    }

    pub struct Digit {
        digit: i32,
    }

    impl Digit {
        pub fn create() -> Self {
            Self { digit: -1 }
        }
    }

    impl Component<Msg> for Digit {
        fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
            info!("{:?}", msg);
            match msg {
                Msg::SelectDigit(d) => self.digit = d,
            }
            Cmd::none()
        }

        fn view(&self) -> Node<Msg> {
            use rand::prelude::*;
            let digit_column_members = ["?", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
            let mut rng = rand::thread_rng();
            let digit_offset: f32 = rng.sample(rand::distributions::Open01);
            let digit_offset = (digit_offset - 0.5) * 0.02;
            let time_offset: i32 = rng.sample(rand::distributions::Uniform::from(-1000..1000));
            let style_string = format!(
                "flex-direction: column; top: -{}em; position: relative; transition: top {}ms cubic-bezier(0,1.2,0.9,1);",
                self.digit as f32 + digit_offset + 1.01,
                1500 + time_offset
            );
            node! {
                <div class="digit-box">
                <div class="digit-column" style={style_string}>
                { for digit in digit_column_members { node!(<div class="digit">{ text(digit) }</div>) } }
                </div>
                </div>
            }
        }
    }
