use glam::{Mat4, Vec3A};
use shared::mesh::ArchivedMesh;

pub struct Actor<const PSIN: usize> {
    pub mesh: &'static ArchivedMesh<PSIN>,
    pub transform: Mat4,

    pub delta: f32,
}

impl<const P: usize> Actor<P> {
    pub fn update(&mut self) {
        // Calculate the y-coordinate using a sine wave
        let y = self.delta.sin();

        // Update the object's position
        self.translate(Vec3A::new(0.0, y * 0.02, 0.0));

        // Increment the angle (delta)
        self.delta += 0.025; // Adjust the increment as needed
    }

    pub fn translate(&mut self, translation: Vec3A) {
        self.transform *= Mat4::from_translation(translation.into());
    }
}
