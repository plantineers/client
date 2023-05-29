use crate::login::PlantBuddyRole;
use crate::management::User;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::hash::Hash;
use std::ops::Range;

const ENDPOINT: &str = "https://pb.mfloto.com/v1/";

/// Represents a temporary user returned by the login API.
#[derive(Deserialize, Debug)]
struct TempUser {
    id: u32,
    name: String,
    role: u64,
}

/// Represents a temporary user used to create a new user.
#[derive(Deserialize, Debug, Serialize, Clone, Default)]
pub struct TempCreationUser {
    pub(crate) name: String,
    pub(crate) password: String,
    pub(crate) role: u64,
}

/// Represents the result of a request.
pub type RequestResult<T> = Result<T, String>;

/// Logs in a user with the given username and password.
///
/// # Arguments
///
/// * `username` - A string slice that holds the username.
/// * `password` - A string slice that holds the password.
///
/// # Returns
///
/// Returns a `TempCreationUser` struct representing the logged-in user.
pub async fn login(username: String, password: String) -> RequestResult<TempCreationUser> {
    info!("Login Server request");
    let client = reqwest::Client::new();
    let response = client
        .get(ENDPOINT.to_string() + "user/login")
        .header(
            "Authorization",
            "Basic ".to_string() + &encode_credentials(username.clone(), password.clone()),
        )
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let result = response.error_for_status_ref().map(|_| ());

    match result {
        Ok(_) => {
            let res = response.text().await.map_err(|e| e.to_string())?;
            let v: Value = serde_json::from_str(&res).unwrap();
            let role_value = v["role"]
                .as_u64()
                .ok_or("Role not found or not an integer")
                .unwrap();

            let login_user = TempCreationUser {
                name: username.clone(),
                password: password.clone(),
                role: role_value.clone(),
            };
            info!("Login successful");
            Ok(login_user)
        }
        Err(e) => {
            info!("Login failed");
            Err(e.to_string())
        }
    }
}

/// Gets all users with the given username and password.
///
/// # Arguments
///
/// * `username` - A string slice that holds the username.
/// * `password` - A string slice that holds the password.
///
/// # Returns
///
/// Returns a vector of `User` structs representing all the users.
pub async fn get_all_users(username: String, password: String) -> RequestResult<Vec<User>> {
    let client = reqwest::Client::new();
    let response = client
        .get(ENDPOINT.to_string() + "users")
        .header(
            "Authorization",
            "Basic ".to_string() + &encode_credentials(username.clone(), password.clone()),
        )
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let result = response.error_for_status_ref().map(|_| ());

    match result {
        Ok(_) => {
            let ids: Vec<i64> = response.json().await.map_err(|e| e.to_string())?;

            let mut users = Vec::new();
            for id in ids {
                let response = client
                    .get(ENDPOINT.to_string() + &format!("user/{}", id))
                    .header(
                        "Authorization",
                        "Basic ".to_string()
                            + &encode_credentials(username.clone(), password.clone()),
                    )
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;

                let temp_user: TempUser = response.json().await.map_err(|e| e.to_string())?;

                let role = PlantBuddyRole::try_from(temp_user.role).unwrap();
                let user = User {
                    id: temp_user.id,
                    name: temp_user.name,
                    role,
                    password: String::new(),
                };

                users.push(user);
            }
            info!("Get all users successful");
            Ok(users)
        }
        Err(e) => {
            info!("Get all users failed");
            Err(e.to_string())
        }
    }
}

/// Creates a new user with the given username, password, and user data.
///
/// # Arguments
///
/// * `username` - A string slice that holds the username.
/// * `password` - A string slice that holds the password.
/// * `user` - A `TempCreationUser` struct representing the user to create.
///
/// # Returns
///
/// Returns a `RequestResult` indicating whether the user was created successfully.
pub async fn create_user(
    username: String,
    password: String,
    user: TempCreationUser,
) -> RequestResult<()> {
    let client = reqwest::Client::new();
    let json = serde_json::to_string(&user).unwrap();
    let response = client
        .post(ENDPOINT.to_string() + "user")
        .header(
            "Authorization",
            "Basic ".to_string() + &encode_credentials(username.clone(), password.clone()),
        )
        .json(&user)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let result = response.error_for_status_ref().map(|_| ());

    match result {
        Ok(_) => {
            info!("Create user successful");
            Ok(())
        }
        Err(e) => {
            info!("Create user failed");
            Err(e.to_string())
        }
    }
}

/// Deletes a user with the given username, password, and ID.
///
/// # Arguments
///
/// * `username` - A string slice that holds the username.
/// * `password` - A string slice that holds the password.
/// * `id` - The ID of the user to delete.
///
/// # Returns
///
/// Returns a `RequestResult` indicating whether the user was deleted successfully.
pub async fn delete_user(username: String, password: String, id: u32) -> RequestResult<()> {
    let client = reqwest::Client::new();
    let response = client
        .delete(ENDPOINT.to_string() + &format!("user/{}", id))
        .header(
            "Authorization",
            "Basic ".to_string() + &encode_credentials(username.clone(), password.clone()),
        )
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let result = response.error_for_status_ref().map(|_| ());

    match result {
        Ok(_) => {
            info!("Delete user successful");
            Ok(())
        }
        Err(e) => {
            info!("Delete user failed");
            Err(e.to_string())
        }
    }
}

/// Updates a user with the given username, password, ID, and user data.
///
/// # Arguments
///
/// * `username` - A string slice that holds the username.
/// * `password` - A string slice that holds the password.
/// * `id` - The ID of the user to update.
/// * `user` - A `TempCreationUser` struct representing the updated user data.
///
/// # Returns
///
/// Returns a `RequestResult` indicating whether the user was updated successfully.
pub async fn update_user(
    username: String,
    password: String,
    id: u32,
    user: TempCreationUser,
) -> RequestResult<()> {
    let client = reqwest::Client::new();
    let response = client
        .put(ENDPOINT.to_string() + &format!("user/{}", id))
        .header(
            "Authorization",
            "Basic ".to_string() + &encode_credentials(username.clone(), password.clone()),
        )
        .json(&user)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let result = response.error_for_status_ref().map(|_| ());

    match result {
        Ok(_) => {
            info!("Update user successful");
            Ok(())
        }
        Err(e) => {
            info!("Update user failed");
            Err(e.to_string())
        }
    }
}

/// Encodes the given username and password as a Base64-encoded string.
///
/// # Arguments
///
/// * `username` - A string slice that holds the username.
/// * `password` - A string slice that holds the password.
///
/// # Returns
///
/// Returns a string representing the encoded credentials.
pub fn encode_credentials(username: String, password: String) -> String {
    let combined = format!("{}:{}", username, password);
    let encoded = general_purpose::STANDARD.encode(combined);
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    #[tokio::test]
    async fn test_login() {
        let username = "testuser".to_string();
        let password = "testpassword".to_string();
        let result = login(username, password).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_all_users() {
        let username = "testuser".to_string();
        let password = "testpassword".to_string();
        let result = get_all_users(username, password).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_user() {
        let username = "testuser2".to_string();
        let password = "testpassword".to_string();
        let random: u32 = random();
        let user = TempCreationUser {
            name: random.to_string(),
            password: "testpassword".to_string(),
            role: PlantBuddyRole::Admin.into(),
        };
        let result = create_user(username, password, user).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_credentials() {
        let username = "testuser".to_string();
        let password = "testpassword".to_string();
        let result = encode_credentials(username, password);
        assert_eq!(result, "dGVzdHVzZXI6dGVzdHBhc3N3b3Jk");
    }
}
