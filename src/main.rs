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
        
        for i in 0..rend.mesh.triangles.len(){
            let tri = rend.mesh.triangles[i];
            let vertex1 = math::mul_vec_mat(&tri.vertices[0], &rend.proj_mat);
            let vertex2 = math::mul_vec_mat(&tri.vertices[1], &rend.proj_mat);
            let vertex3 = math::mul_vec_mat(&tri.vertices[2], &rend.proj_mat);
            let mut tri = renderer::Triangle::new(vertex1, vertex2, vertex3);

            //scaling
            
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
    }
}
