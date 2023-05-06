use std::f32::consts::PI;

use glam::{vec2, vec3, Vec2, Vec2Swizzles, Vec3};

fn pixel_space_to_3d_space(
    pixel_pos1: Vec2,
    pixel_pos2: Vec2,
    camera2_pos_rel: Vec3,
    size: Vec2,
    fov_horizontal: f32,
) -> Vec3 {
    let rel_pixel_pos1 = vec2(
        (pixel_pos1.x - size.x * 0.5) / size.x,
        (pixel_pos1.y - size.x * 0.5) / size.y,
    );
    let rel_pixel_pos2 = vec2(
        (pixel_pos2.x - size.x * 0.5) / size.x,
        (pixel_pos2.y - size.x * 0.5) / size.y,
    );
    let camera_dist = (size.x * 0.5) / (fov_horizontal * 0.5).tan();
    let angles_pos_1 = vec2(
        (rel_pixel_pos1.x / camera_dist).atan(),
        (rel_pixel_pos1.y / camera_dist).atan(),
    );
    let angles_pos_2 = vec2(
        (rel_pixel_pos2.x / camera_dist).atan(),
        (rel_pixel_pos2.y / camera_dist).atan(),
    );

    let angles_cameras = vec3(
        (camera2_pos_rel.z / camera2_pos_rel.x).atan(),
        (camera2_pos_rel.y / camera2_pos_rel.x).atan(),
        (camera2_pos_rel.z / camera2_pos_rel.y).atan(),
    );
    let dist_cameras = camera2_pos_rel.length();
    let dist_camera2_point_xz = dist_cameras
        * ((PI * 0.5) - angles_pos_1.x - angles_cameras.x)
        / (angles_pos_1.x + angles_pos_2.y + PI * 0.5);
    let res_xy = vec2(
        angles_pos_2.y.sin() * dist_camera2_point_xz,
        angles_pos_2.y.cos() * dist_camera2_point_xz,
    );
    res_xy.xyx()
}
