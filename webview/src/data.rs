use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Menu {
    pub id: Option<usize>,

    pub store_id: usize,

    pub name: String,
    pub description: String,
    pub price: f64,
    pub price_unit: String,
    pub timestamp: i64,
    pub image: Option<String>,
    pub rate: usize,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Store {
    pub id: Option<usize>,

    pub map_id: usize,

    pub name: String,
    pub description: String,
    pub xpos: f64,
    pub ypos: f64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Map {
    pub id: Option<usize>,

    pub name: String,
    pub description: String,
    pub image: Option<String>,
}
