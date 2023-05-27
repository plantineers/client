use crate::login::PlantBuddyRole;
use crate::management::User;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::hash::Hash;
use std::ops::Range;

const ENDPOINT: &str = "https://pb.mfloto.com/v1/";

#[derive(Deserialize, Debug)]
struct TempUser {
    id: u32,
    name: String,
    role: u64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct TempCreationUser {
    pub(crate) name: String,
    pub(crate) password: String,
    pub(crate) role: u64,
}

impl Default for TempCreationUser {
    fn default() -> Self {
        TempCreationUser {
            name: String::new(),
            password: String::new(),
            role: PlantBuddyRole::NotLoggedIn.into(),
        }
    }
}

pub type RequestResult<T> = Result<T, String>;

pub async fn login(username: String, password: String) -> RequestResult<TempCreationUser> {
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

            Ok(login_user)
        }
        Err(e) => Err(e.to_string()),
    }
}

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

            println!("{:?}", ids);
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
                println!("{:?}", user);

                users.push(user);
            }
            print!("{:?}", users);
            Ok(users)
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn create_user(
    username: String,
    password: String,
    user: TempCreationUser,
) -> RequestResult<()> {
    let client = reqwest::Client::new();
    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);
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
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

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
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

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
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn encode_credentials(username: String, password: String) -> String {
    let combined = format!("{}:{}", username, password);
    let encoded = general_purpose::STANDARD.encode(combined);
    encoded
}
