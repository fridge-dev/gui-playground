/// Rotate a point (x, y) around a center point (cx, cy) a percentage of a full circle rotation.
// ChatGPT: "rust math equation for rotating an (x,y) coordinate a certain percentage around a center point"
// https://chatgpt.com/c/67973eaf-c1cc-8000-9c7c-5201c97e1681 (private link)
pub fn rotate_point(
    x: f32,
    y: f32,
    center_x: f32,
    center_y: f32,
    percentage: f32, // [0.0, 1.0)
) -> (f32, f32) {
    // Convert percentage to radians
    let angle = percentage * std::f64::consts::TAU as f32;

    // Translate point to origin
    let translated_x = x - center_x;
    let translated_y = y - center_y;

    // Apply rotation matrix
    let rotated_x = translated_x * angle.cos() - translated_y * angle.sin();
    let rotated_y = translated_x * angle.sin() + translated_y * angle.cos();

    // Translate point back
    let result_x = rotated_x + center_x;
    let result_y = rotated_y + center_y;

    (result_x, result_y)
}
