use crate::custom_data_types::matrices::ModelMatrix;

pub struct GameObject {
    pub object_id: usize,
    pub model_matrix: ModelMatrix,
}

impl GameObject {
    pub fn new(object_id: usize, model_matrix: ModelMatrix) -> Self {
        Self {
            object_id,
            model_matrix,
        }
    }
}
