pub enum View {
    Password {
        client_id: String,
        username: String,
        password: String,
    },

    OpenApiAuth {
        client_id: String,
        client_secret: String,
    },
}

impl View {
    fn client_id(&self) -> &str {
        match self {
            View::Password { client_id, .. } => &client_id,
            View::OpenApiAuth { client_id, .. } => &client_id,
        }
    }
}

impl super::ViewVariant for View {
    type Message = Message;

    fn view(
        &self,
        _fields: &crate::AppFields,
    ) -> iced::Element<'_, crate::Message, iced::Renderer<iced::Theme>> {
        let mut elements = Vec::new();

        elements.push(
            iced::widget::text_input("Client ID", self.client_id())
                .on_input(|input| crate::Message::Login(Message::ClientIdChanged(input)))
                .into(),
        );

        match self {
            View::Password {
                username, password, ..
            } => {
                elements.push(
                    iced::widget::text_input("Username", &username)
                        .on_input(|input| crate::Message::Login(Message::UsernameChanged(input)))
                        .into(),
                );
                elements.push(
                    iced::widget::text_input("Password", &password)
                        .on_input(|input| crate::Message::Login(Message::PasswordChanged(input)))
                        .into(),
                );
            }

            View::OpenApiAuth { client_secret, .. } => {
                elements.push(
                    iced::widget::text_input("Client Secret", &client_secret)
                        .on_input(|input| {
                            crate::Message::Login(Message::ClientSecretChanged(input))
                        })
                        .into(),
                );
            }
        }

        elements.push(
            iced::widget::container(
                iced::widget::button(iced::widget::text("Change login method").style(
                    iced::theme::Text::Color(iced::Color::new(0.5, 0.5, 0.5, 1.0)),
                ))
                .style(iced::theme::Button::Text)
                .on_press(crate::Message::Login(Message::SwitchLoginMethod)),
            )
            .width(iced::Length::Fill)
            .height(48)
            .center_x()
            .center_y()
            .into(),
        );

        iced::widget::container(iced::widget::column(elements).width(250))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(
        &mut self,
        message: Self::Message,
        fields: &mut crate::AppFields,
    ) -> (iced::Command<crate::Message>, Option<super::View>) {
        match message {
            Message::ClientIdChanged(cid) => match self {
                View::Password { client_id, .. } => *client_id = cid,
                View::OpenApiAuth { client_id, .. } => *client_id = cid,
            },
            Message::UsernameChanged(usr) => match self {
                View::Password { username, .. } => *username = usr,
                _ => (),
            },
            Message::PasswordChanged(pw) => match self {
                View::Password { password, .. } => *password = pw,
                _ => (),
            },
            Message::ClientSecretChanged(secret) => match self {
                View::OpenApiAuth { client_secret, .. } => *client_secret = secret,
                _ => (),
            },
            Message::SwitchLoginMethod => {
                *self = match self {
                    View::Password { client_id, .. } => Self::OpenApiAuth {
                        client_id: client_id.clone(),
                        client_secret: String::new(),
                    },
                    View::OpenApiAuth { client_id, .. } => Self::Password {
                        client_id: client_id.clone(),
                        username: String::new(),
                        password: String::new(),
                    },
                }
            }
            Message::Login => {}
        }

        (iced::Command::none(), None)
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    ClientIdChanged(String),
    UsernameChanged(String),
    PasswordChanged(String),
    ClientSecretChanged(String),
    SwitchLoginMethod,
    Login,
}
