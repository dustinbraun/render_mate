pub struct Face {
    pub vertex_ids: [u32; 3],
}

impl Face {
    pub fn new(vertex_ids: [u32; 3]) -> Face {
        Face {
            vertex_ids,
        }
    }
}