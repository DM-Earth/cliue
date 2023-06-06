pub mod login;

use std::fmt::Debug;

pub enum View {
    Login(login::View),
}

impl ViewVariant for View {
    type Message = crate::Message;

    fn view(
        &self,
        fields: &crate::AppFields,
    ) -> iced::Element<'_, crate::Message, iced::Renderer<iced::Theme>> {
        match self {
            View::Login(view) => view.view(fields),
        }
    }

    fn update(
        &mut self,
        message: Self::Message,
        fields: &mut crate::AppFields,
    ) -> (iced::Command<Self::Message>, std::option::Option<View>) {
        match self {
            View::Login(login) => match message {
                crate::Message::Login(l) => login.update(l, fields),
            },
        }
    }
}

pub(crate) trait ViewVariant {
    type Message: Clone + Debug;

    fn view(
        &self,
        fields: &crate::AppFields,
    ) -> iced::Element<'_, crate::Message, iced::Renderer<iced::Theme>>;

    fn update(
        &mut self,
        message: Self::Message,
        fields: &mut crate::AppFields,
    ) -> (iced::Command<crate::Message>, Option<View>);
}
