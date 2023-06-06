#[derive(Clone, Copy)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn opposite(&self) -> Self {
        match self {
            Theme::Dark => Self::Light,
            Theme::Light => Self::Dark,
        }
    }

    pub fn as_iced(&self) -> iced::Theme {
        match self {
            Theme::Light => iced::Theme::Light,
            Theme::Dark => iced::Theme::Dark,
        }
    }
}

impl Into<iced::Theme> for Theme {
    fn into(self) -> iced::Theme {
        self.as_iced()
    }
}

impl Default for Theme {
    fn default() -> Self {
        match dark_light::detect() {
            dark_light::Mode::Dark => Self::Dark,
            dark_light::Mode::Light => Self::Light,
            dark_light::Mode::Default => Self::Light,
        }
    }
}
