use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreatePetsForm {
    pub name: String,
    pub description: String,
    pub tutor_id: i32
}
