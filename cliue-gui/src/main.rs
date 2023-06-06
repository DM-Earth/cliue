mod config;
mod theme;
mod view;

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use iced::Application;
use view::ViewVariant;

fn main() {
    CliueApp::run(iced::Settings {
        id: Some("com.dm.earth.cliue".to_string()),
        window: iced::window::Settings {
            size: (800, 600),
            min_size: Some((400, 300)),
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
            platform_specific: iced::window::PlatformSpecific,
            ..Default::default()
        },
        flags: (),
        default_font: None,
        default_text_size: 20.0,
        text_multithreading: false,
        antialiasing: false,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
    .unwrap();
}

struct AppFields {
    pub config: config::Config,
    pub theme: theme::Theme,
}

struct CliueApp {
    fields: RefCell<AppFields>,
    view: view::View,
}

impl Application for CliueApp {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                fields: RefCell::new(AppFields {
                    config: crate::config::Config::new().unwrap(),
                    theme: theme::Theme::default(),
                }),
                view: view::View::Login(view::login::View::Password {
                    client_id: String::new(),
                    username: String::new(),
                    password: String::new(),
                }),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        "Cliue".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        let result = self
            .view
            .update(message, self.fields.borrow_mut().deref_mut());
        match result.1 {
            Some(some) => self.view = some,
            None => (),
        }
        result.0
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        self.view.view(self.fields.borrow().deref())
    }

    fn theme(&self) -> Self::Theme {
        self.fields.borrow().theme.into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Login(view::login::Message),
}
