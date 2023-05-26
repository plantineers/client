use crate::detail::DetailMessage;
use crate::graphs::{PlantChart, PlantCharts};
use crate::login::PlantBuddyRole;
use crate::management::User;
use crate::{graphs, Message};
use env_logger::fmt::Timestamp;
use iced::theme::palette::Primary;
use itertools::Itertools;
use log::info;
use plotters::style::RED;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::hash::Hash;
use std::ops::Range;

const ENDPOINT: &str = "https://pb.mfloto.com/v1/";
pub type RequestResult<T> = Result<T, String>;
pub async fn login(username: String, password: String) -> RequestResult<PlantBuddyRole> {
    let client = reqwest::Client::new();
    let response = client
        .get(ENDPOINT.to_string() + "user/login")
        .header("X-User-Name", username)
        .header("X-User-Password", password)
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
            let role = PlantBuddyRole::try_from(role_value).unwrap();
            Ok(role)
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

#[derive(Deserialize, Debug, Serialize)]
pub struct TempCreationUser {
    pub(crate) name: String,
    pub(crate) password: String,
    pub(crate) role: u64,
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
pub async fn get_all_plant_ids() -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(ENDPOINT.to_string() + "plants")
        .header("X-User-Name", "admin")
        .header("X-User-Password", "1234")
        .send()
        .await?;

    let ids: Vec<String> = response.error_for_status()?.json().await?;
    info!("{:?}", ids);
    Ok(ids)
}
#[derive(Deserialize, Debug)]
pub struct GraphData {
    pub values: Vec<i32>,
    pub timestamps: Vec<String>,
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_graphs(
    plant_ids: Vec<String>,
    sensor_type: String,
) -> Result<Vec<GraphData>, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut graphs = vec![];

    for plant_id in plant_ids {
        let response = client
            .get(&format!(
                "{}sensor-data?sensor={}&plant={}&from=2019-01-01T00:00:00.000Z&to=2023-05-20T00:00:00.000Z",
                ENDPOINT, sensor_type, plant_id
            ))
            .header("X-User-Name", "admin")
            .header("X-User-Password", "1234")
            .send()
            .await?;
        let text = response.text().await?;
        dbg!(text.clone());
        let value: Value = serde_json::from_str(&text).unwrap();
        let data = value.get("data").unwrap();
        let mut values = vec![];
        let mut timestamps = vec![];
        data.as_array().unwrap().iter().for_each(|x| {
            let value = x.get("value").unwrap();
            let timestamp = x.get("timestamp").unwrap();
            values.push(value.as_f64().unwrap() as i32);
            timestamps.push(timestamp.as_str().unwrap().to_string());
        });
        graphs.push(GraphData { values, timestamps })
    }

    Ok(graphs)
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
