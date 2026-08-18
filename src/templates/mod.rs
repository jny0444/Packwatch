pub mod character;
pub mod model;

pub use character::{CharacterTemplate, spawn_character};
pub use model::{FixGltfAlpha, ModelTemplate, SceneModelTemplate, spawn_model, spawn_scene_model};
