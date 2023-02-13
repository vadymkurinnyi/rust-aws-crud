use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
}

pub struct Task {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: bool,
    pub completed: bool,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Builder, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into))]
pub struct User {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Response<R: Serialize, E: Serialize> {
    Result(R),
    Error(E),
}
