use minifb::{Key, Window, WindowOptions};
mod math;
mod renderer;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "3D software renderer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to Open Window");

    let rend = renderer::Renderer::new(WIDTH, HEIGHT);
    let mut time: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                buffer[(y * WIDTH) + x] = 0x0b421a;
            }
        }

        let mut rot_mat_z = math::Mat4::new();
        rot_mat_z.mat[0][0] = time.cos();
        rot_mat_z.mat[0][1] = time.sin();
        rot_mat_z.mat[1][0] = -time.sin();
        rot_mat_z.mat[1][1] = time.cos();
        rot_mat_z.mat[2][2] = 1.0;
        rot_mat_z.mat[3][3] = 1.0;

        let mut rot_mat_x = math::Mat4::new();
        rot_mat_x.mat[0][0] = 1.0;
        rot_mat_x.mat[1][1] = (time * 0.5).cos();
        rot_mat_x.mat[1][2] = (time * 0.5).sin();
        rot_mat_x.mat[2][1] = -(time * 0.5).sin();
        rot_mat_x.mat[2][2] = (time * 0.5).cos();
        rot_mat_x.mat[3][3] = 1.0;

        
        for i in 0..rend.mesh.triangles.len(){
            let mut tri = rend.mesh.triangles[i];
            //rotation
            let vertex1 = math::mul_vec_mat(&tri.vertices[0], &rot_mat_z);
            let vertex2 = math::mul_vec_mat(&tri.vertices[1], &rot_mat_z);
            let vertex3 = math::mul_vec_mat(&tri.vertices[2], &rot_mat_z);

            let mut vertex1 = math::mul_vec_mat(&vertex1, &rot_mat_x);
            let mut vertex2 = math::mul_vec_mat(&vertex2, &rot_mat_x);
            let mut vertex3 = math::mul_vec_mat(&vertex3, &rot_mat_x);

            //transform
            vertex1.z += 3.0;
            vertex2.z += 3.0;
            vertex3.z += 3.0;

            let vertex1 = math::mul_vec_mat(&vertex1, &rend.proj_mat);
            let vertex2 = math::mul_vec_mat(&vertex2, &rend.proj_mat);
            let vertex3 = math::mul_vec_mat(&vertex3, &rend.proj_mat);
            let mut tri = renderer::Triangle::new(vertex1, vertex2, vertex3);

            //we first change the coordinate system from (-1, 1) to (0, 1) by adding 1.0 and multiplying by 0.5
            //and than multiplying by width and height to get the screen coordinates
            
            tri.vertices[0].x += 1.0;
            tri.vertices[0].y += 1.0;
            tri.vertices[1].x += 1.0;
            tri.vertices[1].y += 1.0;
            tri.vertices[2].x += 1.0;
            tri.vertices[2].y += 1.0;
            
            tri.vertices[0].x *= 0.5 * WIDTH as f32;
            tri.vertices[0].y *= 0.5 * HEIGHT as f32;
            tri.vertices[1].x *= 0.5 * WIDTH as f32;
            tri.vertices[1].y *= 0.5 * HEIGHT as f32;
            tri.vertices[2].x *= 0.5 * WIDTH as f32;
            tri.vertices[2].y *= 0.5 * HEIGHT as f32;

            //println!("Tri vertices 1 x: {}, y: {} z: {}\n vertices 2 x: {}, y: {} z: {}\n vertices 3 x: {}, y: {} z: {}", tri.vertices[0].x, tri.vertices[0].y, tri.vertices[0].z, tri.vertices[1].x, tri.vertices[1].y, tri.vertices[1].z, tri.vertices[2].x, tri.vertices[2].y, tri.vertices[2].z);
            /*
            let x = tri.vertices[0].x as usize;
            let y = tri.vertices[0].y as usize - 1;
            buffer[(y * WIDTH) + x] = 0xffffff;

            let x = tri.vertices[1].x as usize;
            let y = tri.vertices[1].y as usize - 1;
            //println!("len {}", (y * WIDTH) + x);
            buffer[(y * WIDTH) + x] = 0xffffff;
            
            let x = tri.vertices[2].x as usize;
            let y = tri.vertices[2].y as usize - 1;
            buffer[(y * WIDTH) + x] = 0xffffff;
            */
            renderer::draw_triangle(&mut buffer, WIDTH, HEIGHT, &tri.vertices[0], &tri.vertices[1], &tri.vertices[2], 0xffffffff);
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        time += 0.1;
    }
}
