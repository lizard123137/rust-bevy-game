use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MushroomType {
    Blue,
    Brown,
    Red,
}