use iced::Renderer;

use crate::gui::types::message::Message;
use crate::translations::translations::{
    language_translation, notifications_translation, style_translation,
};
use crate::utils::types::icon::Icon;
use crate::{Language, StyleType};

/// This enum defines the current running page.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SettingsPage {
    /// Settings Notifications page.
    Notifications,
    /// Settings Appearance page.
    Appearance,
    /// Settings Language page.
    Language,
}

impl SettingsPage {
    pub const ALL: [SettingsPage; 3] = [
        SettingsPage::Notifications,
        SettingsPage::Appearance,
        SettingsPage::Language,
    ];

    pub fn get_tab_label(&self, language: Language) -> &str {
        match self {
            SettingsPage::Notifications => notifications_translation(language),
            SettingsPage::Appearance => style_translation(language),
            SettingsPage::Language => language_translation(language),
        }
    }

    pub fn next(self) -> Self {
        match self {
            SettingsPage::Notifications => SettingsPage::Appearance,
            SettingsPage::Appearance => SettingsPage::Language,
            SettingsPage::Language => SettingsPage::Notifications,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            SettingsPage::Notifications => SettingsPage::Language,
            SettingsPage::Appearance => SettingsPage::Notifications,
            SettingsPage::Language => SettingsPage::Appearance,
        }
    }

    pub fn icon(self) -> iced::advanced::widget::Text<'static, Renderer<StyleType>> {
        match self {
            SettingsPage::Notifications => Icon::Notification,
            SettingsPage::Appearance => Icon::HalfSun,
            SettingsPage::Language => Icon::Globe,
        }
        .to_text()
    }

    pub fn action(self) -> Message {
        Message::OpenSettings(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::gui::pages::types::settings_page::SettingsPage;

    #[test]
    fn test_previous_settings_page() {
        assert_eq!(
            SettingsPage::Notifications.previous(),
            SettingsPage::Language
        );
        assert_eq!(
            SettingsPage::Appearance.previous(),
            SettingsPage::Notifications
        );
        assert_eq!(SettingsPage::Language.previous(), SettingsPage::Appearance);
    }

    #[test]
    fn test_next_settings_page() {
        assert_eq!(SettingsPage::Notifications.next(), SettingsPage::Appearance);
        assert_eq!(SettingsPage::Appearance.next(), SettingsPage::Language);
        assert_eq!(SettingsPage::Language.next(), SettingsPage::Notifications);
    }
}
