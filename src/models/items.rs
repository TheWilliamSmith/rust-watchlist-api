use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct ItemResponse {
    Id: Uuid,
    Title: String,
    Kind: String,
    Status: String;
    ReleaseDate: Date,
    Rating: f32,
    Notes: String,
    CreatedAt: DateTime,
    UpdatedAt: DateTime,
}

#[derive(Deserialize)]
pub struct CreateItemRequest {
    Title: String,
    Kind: String,
    Status: String;
    ReleaseDate: Date,
    Rating: f32,
    Notes: String,
}

#[derive(Deserialize)]
pub struct UpdateItemRequest {
    Title: String,
    Kind: String,
    Status: String;
    ReleaseDate: Date,
    Rating: f32,
    Notes: String,
}
