use render::Render;
use vertex::Vertex;
use glium::{Surface, VertexBuffer};
use glium::backend::Facade;

pub struct World {
    /// Struct representing the game's world. The world is made up of individual `Chunk`s.
    pub chunk: Chunk,
}
impl World {
    pub fn new<F: ?Sized>(facade: &F) -> World
    where
        F: Facade,
    {
        World { chunk: Chunk::new(facade) }
    }
}
impl Render for World {
    fn render<S>(&self, surface: &mut S)
    where
        S: Surface,
    {
        self.chunk.render(surface);
    }
}
const CHUNK_SIZE: usize = 16;
const CHUNK_HEIGHT: usize = 100;
/// A full-height "slice" of the world that is rendered independently.
pub struct Chunk {
    pub blocks: [[[Block; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE],
    vertices: VertexBuffer<Vertex>,
}
impl Chunk {
    pub fn new<F: ?Sized>(facade: &F) -> Chunk
    where
        F: Facade,
    {
        // even though chunks limit `write` calls to only one chunk at a time, a dynamic buffer is
        // better able to manage such "frequent" changes
        let buf = VertexBuffer::dynamic(facade, &[]).unwrap(); //empty for now
        Chunk {
            blocks: [[[0; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE],
            vertices: buf,
        }
    }
    /// Returns a given block with the given **chunk coordinates.** These are relative to the
    /// chunk's origin.
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[x][y][z]
    }
    /// Called whenever an update in the chunk requires the vertex buffer to be updated.
    pub fn update_vertices(&mut self) {}
}
impl Render for Chunk {
    fn render<S>(&self, surface: &mut S)
    where
        S: Surface,
    {
        surface.clear_color_and_depth((1.0, 0.0, 0.0, 1.0), 1.0);
    }
}

pub type Block = u16;
const BLOCK_COLORS: [(f32, f32, f32, f32); 3] = [
    (0.0, 0.0, 0.0, 0.0),
    (0.0, 0.0, 0.0, 1.0),
    (0.0, 1.0, 0.0, 1.0),
];
