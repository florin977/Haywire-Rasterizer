use super::vec4::Vec4;

pub struct Mesh {
    pub vertices: Vec<Vec4>,
    pub normals: Vec<Vec4>,
    pub indices: Vec<usize>,
}

impl Mesh {
    // Loads the vertices, indices and normals for one object in a .obj file
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

            let mut normals = vec![];
            if !mesh_data.normals.is_empty() {
                normals = mesh_data
                    .normals
                    .chunks(3)
                    .map(|n| Vec4::new(n[0], n[1], n[2], 0.0001))
                    .collect();
            }

            let indices: Vec<usize> = mesh_data.indices.iter().map(|&i| i as usize).collect();

            mesh = Self {
                vertices,
                normals,
                indices,
            }
        } else {
            println!("Cannot load the desired mesh type");
            mesh = Self {
                vertices: vec![],
                normals: vec![],
                indices: vec![],
            }
        }

        mesh
    }
}
