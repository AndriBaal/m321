use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<ObjectId>
}
