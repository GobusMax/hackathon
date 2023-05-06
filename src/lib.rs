use cgmath::{prelude::*, vec2, Vector2, Vector3};

fn pixel_space_to_3d_space(
    pixel_pos1: Vector2<f32>,
    pixel_pos2: Vector2<f32>,
    camera2_pos: Vector3<f32>,
    size: Vector2<f32>,
    fov: Vector2<f32>,
) -> Vector3<f32> {
    let rel_pixel_pos1 = vec2(pixel_pos1.x / size.x, pixel_pos1.y / size.y);
    let rel_pixel_pos2 = vec2(pixel_pos2.x / size.x, pixel_pos2.y / size.y);
    let angles1 = todo!();
}
