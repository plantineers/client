use crate::login::PlantBuddyRole;
use crate::management::User;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Range;
use tokio::task::spawn_blocking;

const ENDPOINT: &str = "https://pb.mfloto.com/v1/";

pub async fn login(username: String, password: String) -> Result<PlantBuddyRole, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .post(ENDPOINT.to_string() + "user/login")
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
