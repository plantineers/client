use crate::{Icon, Message, Plantbuddy, Tab};

use iced::widget::vertical_slider::draw;
use iced::widget::{button, container, row, scrollable, Rule};
use iced::Alignment::{Center, End};

use crate::login::PlantBuddyRole;
use crate::requests::{
    create_user, delete_user, get_all_users, update_user, RequestResult, TempCreationUser,
};
use iced::widget::slider::update;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{radio, Button, Column, Container, Row, Text, TextInput},
    Alignment, Color, Element, Length,
};
use iced::{Application, Command, Sandbox, Settings};
use iced_aw::TabLabel;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use rand::random;
use serde::Deserialize;

///This enum represents the various states or actions related to user `management`. process
#[derive(Debug, Clone)]
pub enum ManagementMessage {
    /// Message sent when delete user button is pressed, includes the User ID.
    DeleteUserPressed(u32),
    /// Message sent when username is changed, includes the new username as a string.
    UsernameChanged(String),
    /// Message sent when password is changed, includes the new password as a string.
    PasswordChanged(String),
    /// Message sent when user role is changed, includes the new role as `PlantBuddyRole`.
    RoleChanged(PlantBuddyRole),
    /// Message sent when the create new user button is pressed.
    CreateNewUserPressed,
    /// Message sent when the edit user button is pressed, includes the User details.
    EditUserButton(User),
    /// Message sent when user editing operation is done.
    EditUser,
    /// Message sent when get users button is pressed.
    GetUsersPressed,
    /// Message sent when a new user is created, includes the result of the request.
    UserCreated(RequestResult<()>),
    /// Message sent when a user is deleted, includes the result of the request.
    UserDeleted(RequestResult<()>),
    /// Message sent when users are received, includes a vector of received users.
    UsersReceived(RequestResult<Vec<User>>),
    /// Message sent when a user is edited, includes the result of the request.
    UserEdited(RequestResult<()>),
}

/// A struct representing a user in the application. Each user has a unique ID, a username, password and a role.
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) password: String,
    pub(crate) role: PlantBuddyRole,
}

/// The struct represents a management tab in the application UI.
/// It contains fields for user input (username, password, and role) and for displaying user data (users).
/// The `error_message` field is used to show any error messages to the user.
/// The `editing_user` field is used to store the user being edited (if any).
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ManagementTab {
    username_input: String,
    password_input: String,
    role_input: PlantBuddyRole,
    users: Vec<User>,
    error_message: String,
    editing_user: Option<User>,
    pub logged_in_user: TempCreationUser,
}

impl ManagementTab {
    /// Creates a new instance of ManagementTab with default values.
    pub fn new() -> ManagementTab {
        ManagementTab {
            username_input: String::new(),
            password_input: String::new(),
            role_input: PlantBuddyRole::User,
            users: Vec::new(),
            error_message: String::new(),
            editing_user: None,
            logged_in_user: TempCreationUser::default(),
        }
    }

    /// Updates the state of the ManagementTab based on a given message.
    /// Returns a command to be run by the runtime, such as API calls to create or delete users.
    pub fn update(&mut self, message: ManagementMessage) -> Command<ManagementMessage> {
        let username = self.logged_in_user.name.clone();
        let password = self.logged_in_user.password.clone();

        match message {
            ManagementMessage::UsernameChanged(username) => {
                self.username_input = username;
                self.error_message = String::new();
            }
            ManagementMessage::PasswordChanged(password) => {
                self.password_input = password;
                self.error_message = String::new();
            }
            ManagementMessage::CreateNewUserPressed => {
                // Check if in editing mode
                return if self.editing_user.is_none() {
                    // Creation mode
                    if self.username_input.is_empty() || self.password_input.is_empty() {
                        self.error_message = String::from("Username or password is empty");
                        return Command::none();
                    }
                    create_user_pressed(self.clone(), username.clone(), password.clone())
                } else {
                    // Editing mode
                    if self.username_input.is_empty() || self.password_input.is_empty() {
                        self.error_message = String::from("Username or password is empty");
                        return Command::none();
                    }
                    edit_user_pressed(self.clone(), username.clone(), password.clone())
                };
            }
            ManagementMessage::DeleteUserPressed(id) => {
                self.error_message = String::new();
                return delete_user_pressed(id.clone(), username.clone(), password.clone());
            }
            ManagementMessage::RoleChanged(role) => {
                self.role_input = role;
            }
            ManagementMessage::EditUserButton(user) => {
                self.error_message = String::new();
                self.editing_user = Some(user.clone());

                self.role_input = user.role.clone();
                self.username_input = user.name.clone();
                self.password_input = user.password.clone();
            }
            ManagementMessage::EditUser => {
                self.error_message = String::new();
            }
            ManagementMessage::GetUsersPressed => {
                self.error_message = String::new();
                return get_all_users_pressed(username.clone(), password.clone());
            }
            ManagementMessage::UserCreated(result) => match result {
                Ok(_) => {
                    self.error_message = String::from("User created");
                    self.username_input.clear();
                    self.password_input.clear();
                    return self.update(ManagementMessage::GetUsersPressed);
                }
                Err(e) => {
                    self.error_message = e.to_string();
                }
            },
            ManagementMessage::UserDeleted(result) => match result {
                Ok(_) => {
                    self.error_message = String::from("User deleted");
                    return self.update(ManagementMessage::GetUsersPressed);
                }
                Err(e) => {
                    self.error_message = e.to_string();
                }
            },
            ManagementMessage::UsersReceived(result) => match result {
                Ok(users) => {
                    self.users = users;
                }
                Err(e) => {
                    self.error_message = e.to_string();
                }
            },
            ManagementMessage::UserEdited(result) => match result {
                Ok(_) => {
                    self.error_message = String::from("User edited");
                    self.username_input.clear();
                    self.password_input.clear();
                    self.editing_user = None;
                    return self.update(ManagementMessage::GetUsersPressed);
                }
                Err(e) => {
                    self.error_message = e.to_string();
                }
            },
        }
        Command::none()
    }
}

/// Implementations for the `Tab` trait for `ManagementTab` struct.
impl Tab for ManagementTab {
    type Message = Message;

    /// Returns the title of the tab.
    fn title(&self) -> String {
        String::from("Management")
    }

    /// Returns the label of the tab.
    fn tab_label(&self) -> TabLabel {
        TabLabel::IconText(Icon::Management.into(), self.title())
    }
    /// Returns the view of the tab.
    fn view(&self) -> Element<'_, Self::Message> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title()).size(55))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Center)
            .push(self.content());

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(16)
            .into()
    }
    /// Returns the content of the tab.
    fn content(&self) -> Element<'_, Self::Message> {
        let refresh_row = Container::new(
            Button::new("Refresh")
                .height(Length::from(40))
                .on_press(ManagementMessage::GetUsersPressed)
                .style(iced::theme::Button::Primary),
        )
        .width(Length::from(Length::Fill))
        .align_x(Horizontal::Center);

        let mut user_list = Column::new().width(Length::Fill).height(Length::Fill);
        user_list = user_list.push(
            Row::new()
                .spacing(20)
                .push(
                    Container::new(Text::new("#").size(25))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::FillPortion(1)),
                )
                .push(
                    Container::new(Text::new("Username").size(25))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::FillPortion(1)),
                )
                .push(
                    Container::new(Text::new("Role").size(25))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::FillPortion(1)),
                )
                .push(
                    Container::new(Text::new("Edit").size(25))
                        .center_x()
                        .center_y()
                        .width(Length::FillPortion(1)),
                )
                .push(
                    Container::new(Text::new("Delete").size(25))
                        .center_x()
                        .center_y()
                        .width(Length::FillPortion(1)),
                ),
        );
        for (i, user) in self.users.iter().enumerate() {
            let row = Row::new()
                .height(Length::from(50))
                .spacing(20)
                .push(
                    Container::new(Text::new(user.id.clone().to_string()).size(25))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::FillPortion(1)),
                )
                .push(
                    Container::new(Text::new(&user.name).size(25))
                        .center_x()
                        .center_y()
                        .padding(10)
                        .width(Length::FillPortion(1)),
                )
                .push(
                    Container::new(
                        Text::new(match &user.role {
                            PlantBuddyRole::User => "User",
                            PlantBuddyRole::Admin => "Admin",
                            _ => "This should not happen",
                        })
                        .size(25),
                    )
                    .center_x()
                    .center_y()
                    .padding(10)
                    .width(Length::FillPortion(1)),
                )
                .push(
                    Container::new(
                        Button::new(Text::new("Edit").size(25))
                            .on_press(ManagementMessage::EditUserButton(user.clone()))
                            .width(Length::FillPortion(1)),
                    )
                    .center_x()
                    .center_y(),
                )
                .push(
                    Container::new(
                        Button::new(Text::new("Delete").size(25))
                            .on_press(ManagementMessage::DeleteUserPressed(user.clone().id)),
                    )
                    .center_x()
                    .center_y()
                    .width(Length::FillPortion(1)),
                );

            user_list = user_list.push(row).push(Rule::horizontal(10));
        }

        let scrollable = scrollable::Scrollable::new(user_list)
            .width(Length::Fill)
            .height(Length::Fill);

        let radio_column = Container::new(
            Column::new()
                .height(Length::from(150))
                .width(Length::from(200))
                .padding(20)
                .spacing(10)
                .push(
                    radio(
                        "User",
                        PlantBuddyRole::User,
                        Some(self.role_input),
                        ManagementMessage::RoleChanged,
                    )
                    .size(40),
                )
                .push(
                    radio(
                        "Admin",
                        PlantBuddyRole::Admin,
                        Some(self.role_input),
                        ManagementMessage::RoleChanged,
                    )
                    .size(40),
                ),
        )
        .center_y()
        .center_x();

        let input_row = Row::new()
            .align_items(Center)
            .spacing(20)
            .push(
                Container::new(
                    TextInput::new("Username", &self.username_input)
                        .size(40)
                        .on_input(ManagementMessage::UsernameChanged),
                )
                .center_y()
                .center_x()
                .width(Length::from(650)),
            )
            .push(
                Container::new(
                    TextInput::new("Password", &self.password_input)
                        .size(40)
                        .on_input(ManagementMessage::PasswordChanged),
                )
                .center_y()
                .center_x()
                .width(Length::from(650)),
            )
            .push(radio_column)
            .push(
                Button::new(match self.editing_user {
                    Some(_) => Text::new("Edit User").size(30),
                    None => Text::new("Create User").size(30),
                })
                .on_press(ManagementMessage::CreateNewUserPressed),
            );

        let content = Column::new()
            .spacing(20)
            .push(refresh_row)
            .push(scrollable)
            .push(if self.error_message != String::new() {
                Text::new(&self.error_message).style(Color::from_rgb(1.0, 0.0, 0.0))
            } else {
                Text::new("")
            })
            .push(input_row)
            .align_items(Center);

        let content: Element<'_, ManagementMessage> = Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into();

        content.map(Message::Management)
    }
}

/// Creates a new user based on the provided details and returns a command to create the user.
/// The command will return a message to the update function.
/// # Arguments
/// * `plantbuddy` - The current state of the management tab.
/// * `username` - The username of the user that is creating the new user.
/// * `password` - The password of the user that is creating the new user.
/// # Returns
/// A command to create the user.
fn create_user_pressed(
    plantbuddy: ManagementTab,
    username: String,
    password: String,
) -> Command<ManagementMessage> {
    let user_to_create = TempCreationUser {
        name: plantbuddy.username_input.clone(),
        password: plantbuddy.password_input.clone(),
        role: plantbuddy.role_input.clone().into(),
    };

    Command::perform(
        create_user(username, password, user_to_create),
        ManagementMessage::UserCreated,
    )
}

/// Deletes a user based on the provided details and returns a command to delete the user.
/// The command will return a message to the update function.
/// # Arguments
/// * `id` - The id of the user to delete.
/// * `username` - The username of the user that is deleting the user.
/// * `password` - The password of the user that is deleting the user.
/// # Returns
/// A command to delete the user.
fn delete_user_pressed(id: u32, username: String, password: String) -> Command<ManagementMessage> {
    Command::perform(
        delete_user(username, password, id),
        ManagementMessage::UserDeleted,
    )
}

/// Gets all users based on the provided details and returns a command to get all the users.
/// The command will return a message to the update function.
/// # Arguments
/// * `username` - The username of the user that is getting all the users.
/// * `password` - The password of the user that is getting all the users.
/// # Returns
/// A command to get all the users.
fn get_all_users_pressed(username: String, password: String) -> Command<ManagementMessage> {
    Command::perform(
        get_all_users(username, password),
        ManagementMessage::UsersReceived,
    )
}

/// Updates a user based on the provided details and returns a command to update the user.
/// The command will return a message to the update function.
/// # Arguments
/// * `plantbuddy` - The current state of the management tab.
/// * `username` - The username of the user that is updating the user.
/// * `password` - The password of the user that is updating the user.
/// # Returns
/// A command to update the user.
fn edit_user_pressed(
    plantbuddy: ManagementTab,
    username: String,
    password: String,
) -> Command<ManagementMessage> {
    let user_to_edit = TempCreationUser {
        name: plantbuddy.username_input.clone(),
        password: plantbuddy.password_input.clone(),
        role: plantbuddy.role_input.clone().into(),
    };

    Command::perform(
        update_user(
            username,
            password,
            plantbuddy.editing_user.clone().unwrap().id,
            user_to_edit,
        ),
        ManagementMessage::UserEdited,
    )
}
