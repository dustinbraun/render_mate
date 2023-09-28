#[derive(Copy, Clone)]
pub struct Face {
    pub vertex_ids: [u32; 3],
}

impl Face {
    #[inline(always)]
    pub fn new(vertex_ids: [u32; 3]) -> Face {
        Face {
            vertex_ids,
        }
    }

    #[inline(always)]
    pub fn vertex_ids(&self) -> [u32; 3] {
        self.vertex_ids
    }
}