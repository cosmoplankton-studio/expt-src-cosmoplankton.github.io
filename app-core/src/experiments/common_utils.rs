use nalgebra::{Matrix4, Vector3};


/// Get the translation matrix from the translation values
pub fn matrix_translation(trans_x: f32, trans_y: f32, trans_z: f32) -> Matrix4::<f32> {
    let translate = Matrix4::<f32>::new_translation(&Vector3::new(trans_x, trans_y, trans_z));
    translate
}


/// Get the scaling matrix from the non uniform scaling values
pub fn matrix_scaling(scale_x: f32, scale_y: f32, scale_z: f32) -> Matrix4::<f32> {
    let scale = Matrix4::<f32>::new_nonuniform_scaling(&Vector3::new(scale_x, scale_y, scale_z));
    scale
}


/// Multiply two matrices
pub fn matrix_multiply(mat_a: Matrix4::<f32>, mat_b: Matrix4::<f32>) -> Matrix4::<f32> {
    let multiply = mat_a * mat_b;
    multiply
}


