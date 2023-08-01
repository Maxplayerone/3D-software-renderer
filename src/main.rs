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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                buffer[(y * WIDTH) + x] = 0x63ff7b;
            }
        }
        /*
        for i in 0..rend.mesh.triangles.len(){
            let tri = rend.mesh.triangles[i];
            let vertex1 = mul_vec_mat(&tri.vertices[0], &rend.proj_mat);
            let vertex2 = mul_vec_mat(&tri.vertices[1], &rend.proj_mat);
            let vertex3 = mul_vec_mat(&tri.vertices[2], &rend.proj_mat);
            let tri = Triangle::new(vertex1, vertex2, vertex3);

        }
        */
        renderer::draw_line(&mut buffer, WIDTH, &math::Vec3::new(100.0, 50.0, 0.0), &math::Vec3::new(0.0, 0.0, 0.0), 0xffffffff);

        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
