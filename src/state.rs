use bevy::prelude::*;
use bevy::utils::HashMap;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

/// This has to match the assets/state.yaml file.
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default, Serialize, Deserialize)]
pub enum GameState {
    Splash,
    SplashTest,
    MainMenu,
    Settings,
    Credits,
    #[default]
    Game,
    Exit,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Deref)]
pub struct StateConfig(HashMap<GameState, StateDisplay>);

impl StateConfig {
    pub fn load_str(yaml_str: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(yaml_str)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "display")]
pub enum StateDisplay {
    #[serde(rename = "Splash")]
    Splash(SplashState),
    #[serde(rename = "Menu")]
    Menu(MenuState),
    #[serde(rename = "Game")]
    Game,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplashState {
    pub asset: String,
    pub ms: u64,
    pub next: GameState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuState {
    pub background: Option<String>,
    pub logo: Option<String>,
    pub title: Option<String>,
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: Option<MenuItemId>,
    #[serde(default = "default_selectable")]
    pub selectable: bool,
    #[serde(flatten)]
    pub details: MenuItemDetails,
}

fn default_selectable() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "display")]
pub enum MenuItemDetails {
    #[serde(rename = "Text")]
    Text(MenuTextItem),
    #[serde(rename = "Layout")]
    Layout(MenuLayoutItem),
}

#[derive(Debug, Clone, Serialize, Deserialize, Deref, Hash, PartialEq, Eq)]
pub struct MenuItemId(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuTextItem {
    pub text: String,
    pub next: GameState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MenuLayoutItem {
    Break,
}
