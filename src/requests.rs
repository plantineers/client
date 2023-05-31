use std::sync::Arc;

use crate::login::PlantBuddyRole;
use crate::management::User;
use base64::{
    engine::{self, general_purpose},
    Engine as _,
};
use iced::futures::future::join_all;
use itertools::enumerate;
use log::info;
use reqwest::{Client, Request};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use tokio::sync::Mutex;

/// Our Api client that keeps our client and credentials to avoid reencoding and redoing name resolutions
/// The client is wrapped in an Arc<Mutex<reqwest::Client>> to allow for concurrent access using tokio to avoid deadlocks
#[derive(Clone, Debug)]
pub(crate) struct ApiClient {
    client: Arc<Mutex<reqwest::Client>>,
}

impl ApiClient {
    #[must_use]
    pub fn new(username: String, password: String) -> Self {
        // Make a new client with the given username and password as base64 encoded credentials in the headers
        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!(
                        "Basic {}",
                        encode_credentials(username, password)
                    ))
                    .unwrap(),
                );
                headers
            })
            .build()
            .unwrap();
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }

    /// Gets all users in the database
    /// # Returns
    /// Returns a vector of `User` structs representing all the users.
    pub async fn get_all_users(self) -> RequestResult<Vec<User>> {
        let client = self.client.lock().await;
        let response = client
            .get(ENDPOINT.to_string() + "users")
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
}

const ENDPOINT: &str = "https://pb.mfloto.com/v1/";

/// Represents a temporary user returned by the login API.
#[derive(Deserialize, Debug)]
struct TempUser {
    id: u32,
    name: String,
    role: u64,
}

/// Represents a temporary user used to create a new user.
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct TempCreationUser {
    pub(crate) name: String,
    pub(crate) password: String,
    pub(crate) role: u64,
}
/// Implements the `default` trait for `TempCreationUser`.
impl Default for TempCreationUser {
    fn default() -> Self {
        TempCreationUser {
            name: String::new(),
            password: String::new(),
            role: PlantBuddyRole::NotLoggedIn.into(),
        }
    }
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
pub async fn create_plant(
    new_plant: PlantMetadata,
    plant_group_id: i32,
) -> Result<(), reqwest::Error> {
    let mut json = serde_json::to_value(new_plant).unwrap();
    json["plantGroupId"] = json!(plant_group_id);
    info!("{}", json);
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}plant", ENDPOINT))
        .header("Authorization", "Basic YWRtaW46MTIzNA==")
        .json(&json)
        .send()
        .await?;
    let result = response.error_for_status_ref().map(|_| ());

    match result {
        Ok(_) => {
            info!("Successfully created plant");
            Ok(())
        }
        Err(e) => {
            info!("No Plant created");
            Err(e.to_string())
        }
    }
    .expect("TODO: panic message");

    Ok(())
}
pub async fn create_group(new_group: PlantGroupMetadata) -> Result<(), reqwest::Error> {
    let mut json = serde_json::to_value(new_group.clone()).unwrap();
    for (i, sensor) in enumerate(new_group.sensorRanges.iter()) {
        json["sensorRanges"][i]["sensor"] = json!(sensor.sensorType.name);
    }
    info!("{}", json);
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}plant-group", ENDPOINT))
        .header("Authorization", "Basic YWRtaW46MTIzNA==")
        .json(&json)
        .send()
        .await?;
    let result = response.error_for_status_ref().map(|_| ());

    match result {
        Ok(_) => {
            info!("Successfully created Group");
            Ok(())
        }
        Err(e) => {
            info!("No Group created");
            Err(e.to_string())
        }
    }
    .expect("TODO: panic message");

    Ok(())
}
#[tokio::main(flavor = "current_thread")]
pub async fn get_all_plant_ids() -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(ENDPOINT.to_string() + "plants")
        .header("Authorization", "Basic YWRtaW46MTIzNA==")
        .send()
        .await?;
    let text = response.text().await?;
    info!("{:?}", text);
    let mut ids: Vec<String> = vec![];
    if text != "{\"plants\":null}" {
        let value: Value = serde_json::from_str(&text).unwrap();
        let data = value.get("plants").unwrap();
        data.as_array().unwrap().iter().for_each(|x| {
            ids.push(x.to_string());
        });
    }
    Ok(ids)
}
#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct PlantMetadata {
    pub name: String,
    pub description: String,
    pub species: String,
    pub location: String,
    pub additionalCareTips: Vec<String>,
    #[serde(skip_serializing)]
    pub plantGroup: PlantGroupMetadata,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct PlantGroupMetadata {
    pub name: String,
    pub description: String,
    pub careTips: Vec<String>,
    pub sensorRanges: Vec<SensorRange>,
}
impl Default for PlantGroupMetadata {
    fn default() -> Self {
        PlantGroupMetadata {
            name: String::new(),
            description: String::new(),
            careTips: vec![],
            sensorRanges: vec![
                SensorRange {
                    sensorType: SensorType {
                        name: "soil-moisture".to_string(),
                        unit: "percent".to_string(),
                    },
                    min: 0,
                    max: 0,
                },
                SensorRange {
                    sensorType: SensorType {
                        name: "humidity".to_string(),
                        unit: "percent".to_string(),
                    },
                    min: 0,
                    max: 0,
                },
                SensorRange {
                    sensorType: SensorType {
                        name: "temperature".to_string(),
                        unit: "celcius".to_string(),
                    },
                    min: 0,
                    max: 0,
                },
            ],
        }
    }
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct SensorRange {
    #[serde(skip_serializing)]
    pub sensorType: SensorType,
    pub min: i32,
    pub max: i32,
}
#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct SensorType {
    pub name: String,
    pub unit: String,
}
#[tokio::main(flavor = "current_thread")]
pub async fn get_plant_details(
    plant_id: String,
) -> Result<(PlantMetadata, PlantGroupMetadata), reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(ENDPOINT.to_string() + &format!("plant/{}", plant_id))
        .header("Authorization", "Basic YWRtaW46MTIzNA==")
        .send()
        .await?;

    let details: PlantMetadata = response.error_for_status()?.json().await?;
    let plant_group = details.plantGroup.clone();

    Ok((details, plant_group))
}

#[derive(Deserialize, Debug)]
pub struct GraphData {
    pub values: Vec<i32>,
    pub timestamps: Vec<String>,
}
#[tokio::main(flavor = "current_thread")]
pub async fn get_graphs(
    plant_ids: Vec<String>,
    // FIXME: This should use the enum, implementing the display trait which automatically converts to the string
    sensor_type: String,
) -> RequestResult<Vec<GraphData>> {
    let client = reqwest::Client::new();
    let mut tasks = vec![];

    for plant_id in plant_ids {
        let type_clone = sensor_type.clone();
        let client = client.clone();
        let task = tokio::spawn(async move {
            let response = client
                .get(&format!(
                    "{}sensor-data?sensor={}&plant={}&from=2019-01-01T00:00:00.000Z&to=2023-05-29T23:00:00.000Z",
                    ENDPOINT, type_clone, plant_id
                ))
                // FIXME: We should stop leaking the authentication data here, but for the testing DB it's fine for now
                .header("Authorization", "Basic YWRtaW46MTIzNA==")
                .send()
                .await.map_err(|e| e.to_string())?;

            let text = response.text().await.map_err(|e| e.to_string())?;
            // FIXME: If we can get no data back the return type of our function should be an Option
            if text != "{\"data\":null}" {
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
                Ok(GraphData { values, timestamps })
            } else {
                Err("No data found".to_string())
            }
        });
        tasks.push(task);
    }
    let results = join_all(tasks).await;
    let mut graphs = vec![];
    for result in results {
        match result {
            Ok(Ok(graph_data)) => graphs.push(graph_data),
            _ => {}
        }
    }

    Ok(graphs)
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
    let response = client
        .post(ENDPOINT.to_string() + "user")
        .header(
            "Authorization",
            "Basic".to_string() + &encode_credentials(username.clone(), password.clone()),
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
