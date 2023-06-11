use crate::login::PlantBuddyRole;
use crate::management::User;
use base64::{engine::general_purpose, Engine as _};
use iced::futures::future::join_all;
use itertools::enumerate;
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

/// The endpoint of our API
const ENDPOINT: &str = "https://pb.mfloto.com/v1/";

/// Represents the result of a request.
pub type RequestResult<T> = Result<T, String>;

///`PlantMetadata` struct is used to represent the metadata related to a single plant.
#[derive(Deserialize, Debug, Clone, Default, Serialize, PartialEq)]
pub struct PlantMetadata {
    /// * `name`: A string that represents the name of the plant.
    pub name: String,
    /// * `description`: A string that describes the plant.
    pub description: String,
    /// * `species`: A string that identifies the species of the plant.
    pub species: String,
    /// * `location`: A string that represents where the plant is located.
    pub location: String,
    /// * `additionalCareTips`: A vector of strings, each string representing an additional care tip for the plant.
    pub additionalCareTips: Vec<String>,
    /// * `plantGroup`: An instance of `PlantGroupMetadata` struct which includes metadata about the plant group this plant belongs to. This field is not serialized when the `PlantMetadata` is serialized.
    #[serde(skip_serializing)]
    pub plantGroup: PlantGroupMetadata,
}

/// `PlantGroupMetadata` struct is used to represent the metadata related to a group of plants.
#[derive(Deserialize, Debug, Clone, Serialize, PartialEq)]
pub struct PlantGroupMetadata {
    ///`id`: An integer that represents the id of the plant group. This field is not serialized when the `PlantGroupMetadata` is serialized.
    #[serde(skip_serializing)]
    pub id: i32,
    /// * `name`: A string that represents the name of the plant group.
    pub name: String,
    /// * `description`: A string that describes the plant group.
    pub description: String,
    /// * `careTips`: A vector of strings, each string representing a care tip for the plants in the group.
    pub careTips: Vec<String>,
    /// * `sensorRanges`: A vector of `SensorRange` structs, each representing the acceptable sensor range for the plants in the group.
    pub sensorRanges: Vec<SensorRange>,
}

/// Implements the default trait for PlantGroupMetadata
impl Default for PlantGroupMetadata {
    fn default() -> Self {
        PlantGroupMetadata {
            id: 0,
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
                SensorRange {
                    sensorType: SensorType {
                        name: "light".to_string(),
                        unit: "lux".to_string(),
                    },
                    min: 0,
                    max: 0,
                },
            ],
        }
    }
}

/// Represents a the SensorRagen for a given SensorType
#[derive(Deserialize, Debug, Clone, Default, Serialize, PartialEq)]
pub struct SensorRange {
    #[serde(skip_serializing)]
    pub sensorType: SensorType,
    pub min: i32,
    pub max: i32,
}

/// Represents a sensor type
#[derive(Deserialize, Debug, Clone, Default, Serialize, PartialEq)]
pub struct SensorType {
    pub name: String,
    pub unit: String,
}

/// Represents Graphs data to display
#[derive(Deserialize, Debug, Clone)]
pub struct GraphData {
    pub values: Vec<i32>,
    pub timestamps: Vec<String>,
}

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

/// Our Api client that keeps our client and credentials to avoid reencoding and redoing name resolutions
/// The client is wrapped in an Arc<Mutex<reqwest::Client>> to allow for concurrent access using tokio to avoid deadlocks
#[derive(Clone, Debug)]
pub(crate) struct ApiClient {
    client: Arc<Mutex<Client>>,
}

impl ApiClient {
    /// Creates a new ApiClient
    #[must_use]
    pub fn new(username: String, password: String) -> Self {
        Self {
            client: Arc::new(Mutex::new(Self::build_client(
                username.clone(),
                password.clone(),
            ))),
        }
    }
    /// Builds a new client with the given credentials
    fn build_client(username: String, password: String) -> Client {
        Client::builder()
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
            .unwrap()
    }

    /// Replaces the inner client with a new one with the given credentials
    #[tokio::main(flavor = "current_thread")]
    pub async fn replace_inner(self, username: String, password: String) {
        let new_client = Self::build_client(username, password);
        let mut client_lock = self.client.lock().await;
        *client_lock = new_client
    }

    /// Gets the graphs for the given ids, plant, sensor type and time range
    #[tokio::main(flavor = "current_thread")]
    pub async fn get_graphs(
        self,
        ids: Vec<String>,
        plant: bool,
        sensor_type: String,
        time_range: (String, String),
    ) -> RequestResult<Vec<(GraphData, String)>> {
        let client = self.client.lock().await;
        let mut tasks = vec![];

        for id in ids {
            let type_clone = sensor_type.clone();
            let time_range_clone = time_range.clone();
            info!("Getting time range: {:?}", time_range_clone);
            let client = client.clone();
            let mut parameter = String::new();
            if plant {
                parameter = format!(
                    "{}sensor-data?sensor={}&plant={}&from={}&to={}",
                    ENDPOINT,
                    type_clone,
                    id,
                    time_range_clone.0.clone(),
                    time_range_clone.1.clone()
                );
            } else {
                parameter = format!(
                    "{}sensor-data?sensor={}&plantGroup={}&from={}&to={}",
                    ENDPOINT,
                    type_clone,
                    id,
                    time_range_clone.0.clone(),
                    time_range_clone.1.clone()
                );
            }
            let task = tokio::spawn(async move {
                let response = client
                    .get(parameter)
                    .send()
                    .await
                    .map_err(|e| e.to_string())?;

                let text = response.text().await.map_err(|e| e.to_string())?;
                if text != "{\"data\":null}" {
                    let value: Value = serde_json::from_str(&text).unwrap();
                    let data = value.get("data").unwrap();
                    let mut values = vec![];
                    let mut timestamps = vec![];
                    data.as_array().unwrap().iter().for_each(|x| {
                        if type_clone == "temperature" {
                            let value = x.get("value").unwrap();
                            let timestamp = x.get("timestamp").unwrap();
                            values.push(value.as_f64().unwrap() as i32);
                            timestamps.push(timestamp.as_str().unwrap().to_string());
                        } else {
                            let value = x.get("value").unwrap();
                            let timestamp = x.get("timestamp").unwrap();
                            values.push((value.as_f64().unwrap() * 100.0) as i32);
                            timestamps.push(timestamp.as_str().unwrap().to_string());
                        }
                    });
                    Ok((GraphData { values, timestamps }, id))
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

    /// Creates or updates a plant
    pub async fn create_plant(
        self,
        new_plant: PlantMetadata,
        plant_group_id: i32,
        plant_id: Option<String>,
    ) -> Result<(), reqwest::Error> {
        let client = self.client.lock().await;
        let mut json = serde_json::to_value(new_plant).unwrap();
        json["plantGroupId"] = json!(plant_group_id);
        let response = if plant_id.is_none() {
            let response = client
                .post(&format!("{}plant", ENDPOINT))
                .json(&json)
                .send()
                .await?;
            response
        } else {
            let response = client
                .put(&format!("{}plant/{}", ENDPOINT, plant_id.unwrap()))
                .json(&json)
                .send()
                .await?;
            response
        };
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

    /// Deletes a plant
    pub async fn delete_plant(self, plant_id: String) -> Result<(), reqwest::Error> {
        info!("Plant {} deleted", plant_id);
        let client = self.client.lock().await;
        let response = client
            .delete(&format!("{}plant/{}", ENDPOINT, plant_id))
            .send()
            .await?;
        let result = response.error_for_status_ref().map(|_| ());

        match result {
            Ok(_) => {
                info!("Successfully deleted plant");
                Ok(())
            }
            Err(e) => {
                info!("No Plant deleted");
                Err(e)
            }
        }
    }

    /// Deletes a group
    pub async fn delete_group(self, group_id: String) -> Result<(), reqwest::Error> {
        let client = self.client.lock().await;
        let response = client
            .delete(&format!("{}plant-group/{}", ENDPOINT, group_id))
            .send()
            .await?;
        let result = response.error_for_status_ref().map(|_| ());

        match result {
            Ok(_) => {
                info!("Successfully deleted group");
                Ok(())
            }
            Err(e) => {
                info!("No Group deleted");
                Err(e)
            }
        }
    }

    /// Creates or updates a group
    pub async fn create_group(
        self,
        new_group: PlantGroupMetadata,
        group_id: Option<String>,
    ) -> Result<(), reqwest::Error> {
        let mut json = serde_json::to_value(new_group.clone()).unwrap();

        for (i, sensor) in enumerate(new_group.sensorRanges.iter()) {
            json["sensorRanges"][i]["sensor"] = json!(sensor.sensorType.name);
        }
        info!("Creating group with json: {:?}", json);
        println!("Creating group with json: {:?}", json);
        let client = self.client.lock().await;
        let response = if group_id.is_none() {
            client
                .post(&format!("{}plant-group", ENDPOINT))
                .json(&json)
                .send()
                .await?
        } else {
            client
                .put(&format!("{}plant-group/{}", ENDPOINT, group_id.unwrap()))
                .json(&json)
                .send()
                .await?
        };
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
    pub async fn get_all_plant_ids_names(self) -> Result<Vec<(String, String)>, reqwest::Error> {
        let client = self.client.lock().await;
        let response = client
            .get(ENDPOINT.to_string() + "plants/overview")
            .send()
            .await?;
        let text = response.text().await?;
        let mut ids: Vec<(String, String)> = vec![];
        if text != "{\"plants\":null}" {
            let value: Value = serde_json::from_str(&text).unwrap();
            let data = value.get("plants").unwrap();
            data.as_array().unwrap().iter().for_each(|plant| {
                ids.push((
                    plant.get("id").unwrap().to_string(),
                    plant.get("name").unwrap().to_string(),
                ));
            });
        }
        Ok(ids)
    }
    #[tokio::main(flavor = "current_thread")]
    pub async fn get_all_group_ids_names(self) -> Result<Vec<(String, String)>, reqwest::Error> {
        let client = self.client.lock().await;
        let response = client
            .get(ENDPOINT.to_string() + "plant-groups/overview")
            .send()
            .await?;
        let text = response.text().await?;
        let mut ids: Vec<(String, String)> = vec![];
        if text != "{\"plantGroups\":null}" {
            let value: Value = serde_json::from_str(&text).unwrap();
            let data = value.get("plantGroups").unwrap();
            data.as_array().unwrap().iter().for_each(|plant| {
                ids.push((
                    plant.get("id").unwrap().to_string(),
                    plant.get("name").unwrap().to_string(),
                ));
            });
        }
        Ok(ids)
    }
    #[tokio::main(flavor = "current_thread")]
    pub async fn get_plant_details(
        self,
        plant_id: String,
    ) -> Result<(PlantMetadata, PlantGroupMetadata), reqwest::Error> {
        let client = self.client.lock().await;
        let response = client
            .get(ENDPOINT.to_string() + &format!("plant/{}", plant_id))
            .send()
            .await?;

        let details: PlantMetadata = response.error_for_status()?.json().await?;
        let plant_group = details.plantGroup.clone();

        Ok((details, plant_group))
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
    pub async fn create_user(self, user: TempCreationUser) -> RequestResult<()> {
        let client = self.client.lock().await;
        let response = client
            .post(ENDPOINT.to_string() + "user")
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
    pub async fn delete_user(self, id: u32) -> RequestResult<()> {
        let client = self.client.lock().await;
        let response = client
            .delete(ENDPOINT.to_string() + &format!("user/{}", id))
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
    pub async fn update_user(self, id: u32, user: TempCreationUser) -> RequestResult<()> {
        let client = self.client.lock().await;
        let response = client
            .put(ENDPOINT.to_string() + &format!("user/{}", id))
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
}

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
        let api_client = ApiClient::new(username, password);
        let result = api_client.get_all_users().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_user() {
        let username = "testuser".to_string();
        let password = "testpassword".to_string();
        let api_client = ApiClient::new(username, password);
        let random: u32 = random();
        let user = TempCreationUser {
            name: random.to_string(),
            password: "testpassword".to_string(),
            role: PlantBuddyRole::User.into(),
        };
        let result = api_client.create_user(user).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_credentials() {
        let username = "testuser".to_string();
        let password = "testpassword".to_string();
        let result = encode_credentials(username, password);
        assert_eq!(result, "dGVzdHVzZXI6dGVzdHBhc3N3b3Jk");
    }

    #[tokio::test]
    async fn test_create_plant() {
        let username = "testuser".to_string();
        let password = "testpassword".to_string();
        let api_client = ApiClient::new(username, password);
        let mut new_plant = PlantMetadata::default();
        let random: u32 = random();
        new_plant.name = random.to_string();
        let plant_group_id = 1;
        let result = api_client
            .create_plant(new_plant, plant_group_id, None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_group() {
        let username = "testuser".to_string();
        let password = "testpassword".to_string();
        let api_client = ApiClient::new(username, password);
        let mut new_group = PlantGroupMetadata::default();
        let random: u32 = random();
        new_group.name = random.to_string();
        let result = api_client.create_group(new_group, None).await;
        assert!(result.is_ok());
    }
}
