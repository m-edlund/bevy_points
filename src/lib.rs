use bevy::{
    asset::weak_handle,
    prelude::{Handle, Shader},
};

pub mod material;
pub mod mesh;
pub mod plugin;

pub mod prelude {
    pub use crate::material::PointsMaterial;
    pub use crate::mesh::PointsMesh;
    pub use crate::plugin::PointsPlugin;
}

pub const SHADER_HANDLE: Handle<Shader> = weak_handle!("24afcc18-4e4b-47ea-9aee-c4c36bebd555");
