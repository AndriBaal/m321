use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "_id", skip_serializing)]
    pub id: Option<Uuid>
}
