use crate::custom_data_types::{camera::Camera, game_object::GameObject, mesh::Mesh};

pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub objects: Vec<GameObject>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(meshes: Vec<Mesh>, objects: Vec<GameObject>, camera: Camera) -> Self {
        Self {
            meshes,
            objects,
            camera,
        }
    }
}
