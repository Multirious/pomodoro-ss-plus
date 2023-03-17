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
        #[serde(default)]
        pub loop_schedule: bool,
        #[serde(default)]
        pub start_on_startup: bool,
    }

    #[derive(Debug, Deserialize)]
    pub enum Action {
        StartExistingTimer(String, TimerEndAction),
        StartTimer(Timer, TimerEndAction),
    }

    #[derive(Debug, Deserialize)]
    pub enum TimerEndAction {
        Next,
        WaitForConfirmation,
    }
}

mod event {
    use std::{
        any::{Any, TypeId},
        collections::hash_map::DefaultHasher,
        fmt,
        hash::{Hash, Hasher},
        rc::Rc,
    };

    pub struct EventsBuilder(Vec<Event>);

    impl EventsBuilder {
        pub fn new() -> EventsBuilder {
            EventsBuilder(Vec::new())
        }

        pub fn push(&mut self, with: Event) {
            self.0.push(with)
        }
    }

    #[derive(Debug)]
    pub struct Events(Vec<Event>);

    impl Events {
        pub fn builder() -> EventsBuilder {
            EventsBuilder::new()
        }

        pub fn contains(&self, event: &Event) -> bool {
            self.0.contains(event)
        }

        pub fn find<E: 'static>(&self) -> Option<&Event> {
            for e in &self.0 {
                if e.is::<E>() {
                    return Some(e);
                }
            }
            None
        }
    }

    pub trait EventTrait: fmt::Debug {
        fn eq(&self, other: &dyn EventTrait) -> bool;
        fn hash(&self) -> u64;
        // see https://stackoverflow.com/a/33687996/1600898
        fn as_any(&self) -> &dyn Any;
    }

    impl<T: Eq + Hash + fmt::Debug + 'static> EventTrait for T {
        fn eq(&self, other: &dyn EventTrait) -> bool {
            if let Some(other) = other.as_any().downcast_ref::<T>() {
                return self == other;
            }
            false
        }

        fn hash(&self) -> u64 {
            let mut h = DefaultHasher::new();
            // mix the typeid of T into the hash to make distinct types
            // provide distinct hashes
            Hash::hash(&(TypeId::of::<T>(), self), &mut h);
            h.finish()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[derive(Debug)]
    pub struct Event(Rc<dyn EventTrait>);

    impl PartialEq for Event {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&*other.0)
        }
    }

    impl Eq for Event {}

    impl Event {
        pub fn new(event: Rc<dyn EventTrait>) -> Self {
            Event(event)
        }

        pub fn is<E: 'static>(&self) -> bool {
            self.0.as_any().is::<E>()
        }

        pub fn downcast<E: 'static>(&self) -> Option<&E> {
            self.0.as_any().downcast_ref()
        }
    }
}

use std::time::Duration;

use error::Result;
use event::{Event, Events};

fn main() {
    loop {
        let events = Events::builder();
        if let Some(true) = crossterm::event::poll(Duration::ZERO) {}
    }
}
