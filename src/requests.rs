use crate::login::PlantBuddyRole;
use crate::management::User;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::hash::Hash;
use std::ops::Range;

const ENDPOINT: &str = "https://pb.mfloto.com/v1/";

pub type RequestResult<T> = Result<T, String>;

pub async fn login(username: String, password: String) -> RequestResult<TempCreationUser> {
    let client = reqwest::Client::new();
    let response = client
        .get(ENDPOINT.to_string() + "user/login")
        .header("X-User-Name", username.clone())
        .header("X-User-Password", password.clone())
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

#[tokio::main(flavor = "current_thread")]
pub async fn get_all_users(
    username: String,
    password: String,
) -> Result<Vec<User>, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(ENDPOINT.to_string() + "users")
        .header("X-User-Name", &username)
        .header("X-User-Password", &password)
        .send()
        .await?;

    let ids: Vec<i64> = response.error_for_status()?.json().await?;

    println!("{:?}", ids);
    let mut users = Vec::new();
    for id in ids {
        let response = client
            .get(ENDPOINT.to_string() + &format!("user/{}", id))
            .header("X-User-Name", &username)
            .header("X-User-Password", &password)
            .send()
            .await?;

        let temp_user: TempUser = response.error_for_status()?.json().await?;
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
#[tokio::main(flavor = "current_thread")]
pub async fn create_user(
    username: String,
    password: String,
    user: TempCreationUser,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);
    let response = client
        .post(ENDPOINT.to_string() + "user/")
        .header("X-User-Name", &username)
        .header("X-User-Password", &password)
        .json(&user)
        .send()
        .await?;

    let result = response.error_for_status_ref().map(|_| ());

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn delete_user(
    username: String,
    password: String,
    id: u32,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .delete(ENDPOINT.to_string() + &format!("user/{}", id))
        .header("X-User-Name", &username)
        .header("X-User-Password", &password)
        .send()
        .await?;

    let result = response.error_for_status_ref().map(|_| ());

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn update_user(
    username: String,
    password: String,
    id: u32,
    user: TempCreationUser,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .put(ENDPOINT.to_string() + &format!("user/{}", id))
        .header("X-User-Name", &username)
        .header("X-User-Password", &password)
        .json(&user)
        .send()
        .await?;

    let result = response.error_for_status_ref().map(|_| ());

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
