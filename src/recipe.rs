use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub ingredients: Vec<string>,
    pub instructions: Vec<string>,
    pub servings: u32,
}

impl Recipe {
    pub fn new(
        id: u32,
        name: String,
        ingredients: Vec<String>,
        instructions: Vec<String>,
        server: u32,
    ) -> Self {
        Recipe {
            id,
            name,
            ingredients,
            instructions,
            servings,
        }
    }
}
