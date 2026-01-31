use super::vec4::Vec4;

pub struct Mesh {
    pub vertices: Vec<Vec4>,
    pub indices: Vec<usize>,
}

impl Mesh {
    pub fn new(path: &str, obj_type: &str) -> Self {
        let mesh: Mesh;
        if obj_type == ".obj" {
            let load_options = tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            };

            let (models, _materials) =
                tobj::load_obj(path, &load_options).expect("Failed to load models");

            let model = &models[0];
            let mesh_data = &model.mesh;

            let vertices: Vec<Vec4> = mesh_data
                .positions
                .chunks(3)
                .map(|v| Vec4::new(v[0], v[1], v[2], 1.0))
                .collect();

            let indices: Vec<usize> = mesh_data.indices.iter().map(|&i| i as usize).collect();

            mesh = Self { vertices, indices }
        } else {
            println!("Cannot load the desired mesh type");
            mesh = Self {
                vertices: vec![],
                indices: vec![],
            }
        }

        mesh
    }
}
