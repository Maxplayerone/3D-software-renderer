#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub struct Mat4 {
    pub mat: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn new() -> Self {
        Self {
            mat: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }
}

pub fn mul_vec_mat(v: &Vec3, m: &Mat4) -> Vec3 {
    let x = v.x * m.mat[0][0] + v.y * m.mat[1][0] + v.z * m.mat[2][0] + 1.0 * m.mat[3][0];
    let y = v.x * m.mat[0][1] + v.y * m.mat[1][1] + v.z * m.mat[2][1] + 1.0 * m.mat[3][1];
    let z = v.x * m.mat[0][2] + v.y * m.mat[1][2] + v.z * m.mat[2][2] + 1.0 * m.mat[3][2];
    let w = v.x * m.mat[0][3] + v.y * m.mat[1][3] + v.z * m.mat[2][3] + 1.0 * m.mat[3][3];
    let mut vec = Vec3::new(x, y, z);
    if w != 0.0 {
        vec.x = vec.x / w;
        vec.y = vec.y / w;
        vec.z = vec.z / w;
    }
    vec
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    let mut vec = Vec3::new(0.0, 0.0, 0.0);
    vec.x = v1.y * v2.z - v1.z * v2.y;
    vec.y = v1.z * v2.x - v1.x * v2.z;
    vec.z = v1.x * v2.y - v1.y * v2.x;
    vec
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32{
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn normalize(mut v: Vec3) -> Vec3 {
    let length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    v.x = v.x / length;
    v.y = v.y / length;
    v.z = v.z / length;
    v
}

pub fn rad(val_in_degrees: f32) -> f32 {
    val_in_degrees / 180.0 * 3.14159
}

pub fn abs(val: f32) -> f32 {
    if val >= 0.0 {
        val
    } else {
        val * -1.0
    }
}
