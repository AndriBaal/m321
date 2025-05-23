use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeviceLog {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>,
    pub client_name: String,
    pub temperature: i32,
    pub humidity: i32
}
