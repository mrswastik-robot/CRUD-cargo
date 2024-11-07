use crate::manager::RecipeManager;
use crate::recipe::Recipe;
use iced::theme;
use iced::widget::{Button, Column, Container, Row, Scrollable, Text, TextInput};
use iced::Color;
use iced::{Element, Length, Sandbox, Settings};

pub struct RecipeManagerGUI {
    recipe_manager: RecipeManager,
    recipe_name: String,
    recipe_ingredients: String,
    recipe_instructions: String,
    recipe_servings: String,
    selected_recipe: Option<Recipe>,
    error_message: Option<String>,
    editing: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddRecipe,
    EditRecipe(u32),
    UpdateRecipe,
    CancelEdit,
    RecipeNameChanged(String),
    RecipeIngredientsChanged(String),
    RecipeInstructionsChanged(String),
    RecipeServingsChanged(String),
    RecipeSelected(Recipe),
    DeleteRecipe(u32),
    SaveRecipes,
    LoadRecipes,
}
