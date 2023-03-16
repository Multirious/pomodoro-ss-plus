mod error {
    pub use anyhow::Result;
}

mod utils {
    #[derive(Debug, Default, Clone, Copy)]
    pub struct W<T>(pub T);
}

mod cfg {
    use anyhow::Context;
    use crossterm::style::Color;
    use serde::Deserialize;

    use std::time::Duration;

    use crate::{error::Result, utils::W};

    impl<'de> serde::de::Deserialize<'de> for W<Color> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(Deserialize)]
            enum _Color {
                Black,
                DarkGrey,
                Red,
                DarkRed,
                Green,
                DarkGreen,
                Yellow,
                DarkYellow,
                Blue,
                DarkBlue,
                Magenta,
                DarkMagenta,
                Cyan,
                DarkCyan,
                White,
                Grey,
                Rgb { r: u8, g: u8, b: u8 },
                AnsiValue(u8),
            }

            impl _Color {
                pub fn into_crossterm(self) -> Color {
                    match self {
                        _Color::Black => Color::Black,
                        _Color::DarkGrey => Color::DarkGrey,
                        _Color::Red => Color::Red,
                        _Color::DarkRed => Color::DarkRed,
                        _Color::Green => Color::Green,
                        _Color::DarkGreen => Color::DarkGreen,
                        _Color::Yellow => Color::Yellow,
                        _Color::DarkYellow => Color::DarkYellow,
                        _Color::Blue => Color::Blue,
                        _Color::DarkBlue => Color::DarkBlue,
                        _Color::Magenta => Color::Magenta,
                        _Color::DarkMagenta => Color::DarkMagenta,
                        _Color::Cyan => Color::Cyan,
                        _Color::DarkCyan => Color::DarkCyan,
                        _Color::White => Color::White,
                        _Color::Grey => Color::Grey,
                        _Color::Rgb { r, g, b } => Color::Rgb { r, g, b },
                        _Color::AnsiValue(v) => Color::AnsiValue(v),
                    }
                }
            }

            _Color::deserialize(deserializer).map(|color| W(_Color::into_crossterm(color)))
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Cfg {
        pub timers: Vec<Timer>,
        pub schedules: Vec<Schedule>,
    }

    impl Cfg {
        pub fn from_str(s: &str) -> Result<Cfg> {
            toml::from_str(s).with_context(|| "error parsing config")
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Timer {
        pub title: String,
        #[serde(default)]
        pub lock_input: bool,
        pub warn: Option<Duration>,
        pub time: Duration,
        pub theme: Option<Theme>,
        pub sound: Option<std::path::PathBuf>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Theme {
        pub primary_color: W<Color>,
        pub secondary_color: W<Color>,
        pub gradient_bar: bool,
    }

    impl Default for Theme {
        fn default() -> Self {
            Theme {
                primary_color: W(Color::Grey),
                secondary_color: W(Color::Grey),
                gradient_bar: true,
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Schedule {
        pub title: String,
        pub actions: Vec<Action>,
        #[serde(default, rename = "loop")]
        pub loop_schedule: bool,
    }

    #[derive(Debug, Deserialize)]
    pub enum Action {
        StartTimer(String, TimerEndAction),
    }

    #[derive(Debug, Deserialize)]
    pub enum TimerEndAction {
        Next,
        WaitForConfirmation,
    }
}

fn main() {}
