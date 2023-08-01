use crate::math;

#[derive(Copy, Clone)]
struct Triangle {
    vertices: [math::Vec3; 3],
}

impl Triangle {
    pub fn new(v1: math::Vec3, v2: math::Vec3, v3: math::Vec3) -> Self {
        Self {
            vertices: [v1, v2, v3],
        }
    }
}

struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }

    fn push(&mut self, triangle: Triangle) {
        self.triangles.push(triangle)
    }
}

pub struct Renderer {
    mesh: Mesh,
    //projection matrix stuff
    znear: f32,
    zfar: f32,
    fov: f32,
    fov_rad: f32,
    aspect_ratio: f32,
    proj_mat: math::Mat4,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let znear = 0.1;
        let zfar = 1000.0;
        let fov = 90.0;
        let aspect_ratio = width as f32 / height as f32;
        let fov_rad = 1.0 / math::rad(fov * 0.5).tan();

        //[a * fov_rad, 0      , 0                           , 0]
        //[0          , fov_rad, 0                           , 0]
        //[0          , 0      , zfar / (zfar - znear)       , 1]
        //[0          , 0      , -zfar * znear/(zfar - znear), 0]
        //[column][row]
        let mut proj_mat = math::Mat4::new();
        proj_mat.mat[0][0] = aspect_ratio * fov_rad;
        proj_mat.mat[1][1] = fov_rad;
        proj_mat.mat[2][2] = zfar / (zfar - znear);
        proj_mat.mat[3][2] = (-zfar * znear) / (zfar - znear);
        proj_mat.mat[2][3] = 1.0;
        proj_mat.mat[3][3] = 0.0;

        let mut mesh = Mesh::new();
        //south
        mesh.push(Triangle::new(
            math::Vec3::new(0.0, 0.0, 0.0),
            math::Vec3::new(0.0, 1.0, 0.0),
            math::Vec3::new(1.0, 1.0, 0.0),
        ));
        mesh.push(Triangle::new(
            math::Vec3::new(0.0, 0.0, 0.0),
            math::Vec3::new(1.0, 1.0, 0.0),
            math::Vec3::new(1.0, 0.0, 0.0),
        ));
        //east
        mesh.push(Triangle::new(
            math::Vec3::new(1.0, 0.0, 0.0),
            math::Vec3::new(1.0, 1.0, 0.0),
            math::Vec3::new(1.0, 1.0, 1.0),
        ));
        mesh.push(Triangle::new(
            math::Vec3::new(1.0, 0.0, 0.0),
            math::Vec3::new(1.0, 1.0, 1.0),
            math::Vec3::new(1.0, 0.0, 1.0),
        ));
        //north
        mesh.push(Triangle::new(
            math::Vec3::new(1.0, 0.0, 0.0),
            math::Vec3::new(1.0, 1.0, 1.0),
            math::Vec3::new(0.0, 1.0, 1.0),
        ));
        mesh.push(Triangle::new(
            math::Vec3::new(1.0, 0.0, 1.0),
            math::Vec3::new(0.0, 1.0, 1.0),
            math::Vec3::new(0.0, 0.0, 1.0),
        ));
        //west
        mesh.push(Triangle::new(
            math::Vec3::new(0.0, 0.0, 1.0),
            math::Vec3::new(0.0, 1.0, 1.0),
            math::Vec3::new(0.0, 1.0, 0.0),
        ));
        mesh.push(Triangle::new(
            math::Vec3::new(0.0, 0.0, 1.0),
            math::Vec3::new(0.0, 1.0, 0.0),
            math::Vec3::new(0.0, 0.0, 0.0),
        ));
        //top
        mesh.push(Triangle::new(
            math::Vec3::new(0.0, 1.0, 0.0),
            math::Vec3::new(0.0, 1.0, 1.0),
            math::Vec3::new(1.0, 1.0, 1.0),
        ));
        mesh.push(Triangle::new(
            math::Vec3::new(0.0, 1.0, 0.0),
            math::Vec3::new(1.0, 1.0, 1.0),
            math::Vec3::new(1.0, 1.0, 0.0),
        ));
        //top
        mesh.push(Triangle::new(
            math::Vec3::new(1.0, 0.0, 1.0),
            math::Vec3::new(0.0, 0.0, 1.0),
            math::Vec3::new(0.0, 0.0, 0.0),
        ));
        mesh.push(Triangle::new(
            math::Vec3::new(1.0, 0.0, 1.0),
            math::Vec3::new(0.0, 0.0, 0.0),
            math::Vec3::new(1.0, 0.0, 0.0),
        ));

        Self {
            znear,
            zfar,
            fov,
            aspect_ratio,
            fov_rad,
            proj_mat,
            mesh,
        }
    }
}

pub fn draw_line(buffer: &mut Vec<u32>, width: usize, v1: &math::Vec3, v2: &math::Vec3, color: u32) {
    let mut x = v1.x;
    let mut y = v1.y;

    let mut dx = v2.x - v1.x;
    let mut dy = v2.y - v1.y;
    let step;

    if math::abs(dx) >= math::abs(dy) {
        step = math::abs(dx);
    } else {
        step = math::abs(dy);
    }
    dx = dx / step;
    dy = dy / step;

    let mut i = 0;
    while i < step as i32 {
        let mut x_usize: usize = x as usize;
        let mut y_usize: usize = y as usize;
        //println!("X usize: {}. Check index {}", x_usize, y_usize);
        buffer[(y_usize * width) + x_usize] = color;
        x = x + dx;
        y = y + dy;
        i = i + 1;
    }
}
