use glium::Surface;

/// Something "renderable"--that is, it can do operations on the window's Surface to make visual
/// changes.
pub trait Render {
    fn render<S>(&self, target: &mut S)
    where
        S: Surface;
}
