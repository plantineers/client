use crate::{Icon, Message, Tab};
use iced::{
    widget::{Column, Container, Radio, Text},
    Element,
};
use iced_aw::style::TabBarStyles;
use iced_aw::tab_bar::TabLabel;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabBarPosition {
    #[default]
    Top,
    Bottom,
}

impl TabBarPosition {
    pub const ALL: [TabBarPosition; 2] = [TabBarPosition::Top, TabBarPosition::Bottom];
}

impl From<TabBarPosition> for String {
    fn from(position: TabBarPosition) -> Self {
        String::from(match position {
            TabBarPosition::Top => "Top",
            TabBarPosition::Bottom => "Bottom",
        })
    }
}

//#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabSettings {
    pub tab_bar_position: Option<TabBarPosition>,
    pub tab_bar_theme: Option<TabBarStyles>,
}

impl TabSettings {
    pub fn new() -> Self {
        TabSettings {
            tab_bar_position: Some(TabBarPosition::Top),
            tab_bar_theme: Some(TabBarStyles::Green),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    PositionSelected(TabBarPosition),
    ThemeSelected(TabBarStyles),
}

pub struct SettingsTab {
    settings: TabSettings,
}

impl SettingsTab {
    pub fn new() -> Self {
        SettingsTab {
            settings: TabSettings::new(),
        }
    }

    pub fn settings(&self) -> &TabSettings {
        &self.settings
    }

    pub fn update(&mut self, message: SettingsMessage) {
        match message {
            SettingsMessage::PositionSelected(position) => {
                self.settings.tab_bar_position = Some(position)
            }
            SettingsMessage::ThemeSelected(theme) => self.settings.tab_bar_theme = Some(theme),
        }
    }
}

impl Tab for SettingsTab {
    type Message = Message;

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::CogAlt.into(), self.title())
    }

    fn content(&self) -> Element<'_, Self::Message> {
        let content: Element<'_, SettingsMessage> = Container::new(
            Column::new()
                .push(Text::new("TabBar position:").size(35))
                .push(TabBarPosition::ALL.iter().cloned().fold(
                    Column::new().padding(10).spacing(10),
                    |column, position| {
                        column.push(
                            Radio::new(
                                position,
                                position,
                                self.settings().tab_bar_position,
                                SettingsMessage::PositionSelected,
                            )
                            .size(35),
                        )
                    },
                ))
                .push(Text::new("TabBar color:").size(35))
                .push(
                    (0..5).fold(Column::new().padding(10).spacing(10), |column, id| {
                        column.push(
                            Radio::new(
                                predefined_style(id),
                                predefined_style(id),
                                self.settings().tab_bar_theme,
                                SettingsMessage::ThemeSelected,
                            )
                            .size(35),
                        )
                    }),
                ),
        )
        .into();

        content.map(Message::Settings)
    }
}

fn predefined_style(index: usize) -> TabBarStyles {
    match index {
        0 => TabBarStyles::Default,
        1 => TabBarStyles::Red,
        2 => TabBarStyles::Blue,
        3 => TabBarStyles::Green,
        4 => TabBarStyles::Purple,
        _ => TabBarStyles::Default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_settings_new() {
        let settings = TabSettings::new();
        assert_eq!(settings.tab_bar_position, Some(TabBarPosition::Top));
        assert_eq!(settings.tab_bar_theme, Some(TabBarStyles::Green));
    }

    #[test]
    fn test_settings_tab_new() {
        let settings_tab = SettingsTab::new();
        assert_eq!(
            settings_tab.settings().tab_bar_position,
            Some(TabBarPosition::Top)
        );
        assert_eq!(
            settings_tab.settings().tab_bar_theme,
            Some(TabBarStyles::Green)
        );
    }

    #[test]
    fn test_settings_tab_update_position() {
        let mut settings_tab = SettingsTab::new();
        settings_tab.update(SettingsMessage::PositionSelected(TabBarPosition::Bottom));
        assert_eq!(
            settings_tab.settings().tab_bar_position,
            Some(TabBarPosition::Bottom)
        );
    }

    #[test]
    fn test_settings_tab_update_theme() {
        let mut settings_tab = SettingsTab::new();
        settings_tab.update(SettingsMessage::ThemeSelected(TabBarStyles::Red));
        assert_eq!(
            settings_tab.settings().tab_bar_theme,
            Some(TabBarStyles::Red)
        );
    }

    #[test]
    fn test_settings_tab_title() {
        let settings_tab = SettingsTab::new();
        assert_eq!(settings_tab.title(), String::from("Settings"));
    }

    #[test]
    fn test_predefined_style() {
        assert_eq!(predefined_style(0), TabBarStyles::Default);
        assert_eq!(predefined_style(1), TabBarStyles::Red);
        assert_eq!(predefined_style(2), TabBarStyles::Blue);
        assert_eq!(predefined_style(3), TabBarStyles::Green);
        assert_eq!(predefined_style(4), TabBarStyles::Purple);
        assert_eq!(predefined_style(100), TabBarStyles::Default); // Testing an index out of range should return the Default style.
    }
}
