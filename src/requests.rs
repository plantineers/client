use crate::login::PlantBuddyRole;
use crate::management::User;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Range;
use tokio::task::spawn_blocking;

const ENDPOINT: &str = "https://pb.mfloto.com/v1/";

#[tokio::main]
pub async fn login(username: String, password: String) -> Result<PlantBuddyRole, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(ENDPOINT.to_string() + "user/login")
        .header("X-User-Name", username)
        .header("X-User-Password", password)
        .send()
        .await?;

    let result = response.error_for_status_ref().map(|_| ());

    match result {
        Ok(_) => {
            let res = response.text().await?;
            let v: Value = serde_json::from_str(&res).unwrap();
            let role_value = v["role"]
                .as_i64()
                .ok_or("Role not found or not an integer")
                .unwrap();
            let role = PlantBuddyRole::try_from(role_value).unwrap();
            Ok(role)
        }
        Err(e) => Err(e),
    }
}

#[derive(Deserialize, Debug)]
struct TempUser {
    id: u32,
    name: String,
    role: i64,
}

#[tokio::main]
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
