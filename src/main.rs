extern crate minifb;

mod engine;

use minifb::{Key, Window, WindowOptions};
use engine::{draw_line, draw_triangle, Point};

const SCREEN_HEIGHT: f64 = 720.0;
const SCREEN_WIDTH: f64 = 1280.0;

#[derive(Copy, Clone)]
struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone)]
struct Triangle {
    p: [Vec3D; 3],
}

struct Mesh {
    tris: Vec<Triangle>,
}

struct Mat4x4 {
    m: [[f64; 4]; 4],
}

fn build_vertex(x: f64, y: f64, z: f64) -> Vec3D {
    Vec3D { x: x, y: y, z: z }
}

fn build_triangle(v: [f64; 9]) -> Triangle {
    Triangle {
        p: [
            build_vertex(v[0], v[1], v[2]),
            build_vertex(v[3], v[4], v[5]),
            build_vertex(v[6], v[7], v[8]),
        ],
    }
}

fn multiply_matvec(i: &Vec3D, m: &Mat4x4) -> Vec3D {
    let mut o = Vec3D {
        x: 0f64,
        y: 0f64,
        z: 0f64,
    };
    o.y = i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + m.m[3][1];
    o.z = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + m.m[3][2];
    o.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + m.m[3][0];
    let w: f64 = i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + m.m[3][3];

    if w != 0f64 {
        o.x /= w;
        o.y /= w;
        o.z /= w;
    }
    o
}

fn main() {
    let mut mesh_cube = Mesh {
        tris: Vec::new() as Vec<Triangle>,
    };
    let mut mat_proj = Mat4x4 { m: [[0f64; 4]; 4] };

    let camera = build_vertex(0f64, 0f64, 0f64);

    const ASPECT_RATIO: f64 = SCREEN_HEIGHT / SCREEN_WIDTH;

    mesh_cube.tris.push(build_triangle([
        0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0,
    ]));
    mesh_cube.tris.push(build_triangle([
        0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    ]));
    mesh_cube.tris.push(build_triangle([
        1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0,
    ]));
    mesh_cube.tris.push(build_triangle([
        1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0,
    ]));
    mesh_cube.tris.push(build_triangle([
        1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0,
    ]));
    mesh_cube.tris.push(build_triangle([
        1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0,
    ]));
    mesh_cube.tris.push(build_triangle([
        0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0,
    ]));
    mesh_cube.tris.push(build_triangle([
        0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
    ]));
    mesh_cube.tris.push(build_triangle([
        0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ]));
    mesh_cube.tris.push(build_triangle([
        0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
    ]));
    mesh_cube.tris.push(build_triangle([
        1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
    ]));
    mesh_cube.tris.push(build_triangle([
        1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    ]));

    let f_near = 0.1f64;
    let f_far = 1000.0f64;
    let f_fov = 90.0f64;
    let f_asprat = ASPECT_RATIO;
    let f_fovrad = 1.0f64 / (f_fov * 0.5f64 / 180.0f64 * 3.14159f64).tan();

    mat_proj.m[0][0] = f_asprat * f_fovrad;
    mat_proj.m[1][1] = f_fovrad;
    mat_proj.m[2][2] = f_far / (f_far - f_near);
    mat_proj.m[3][2] = (-f_far * f_near) / (f_far - f_near);
    mat_proj.m[2][3] = 1.0f64;
    mat_proj.m[3][3] = 0.0f64;

    let mut elapsed_time = 0f64;

    let mut minifbwin = Window::new(
        "UK3DRen - ESC to exit",
        SCREEN_WIDTH as usize,
        SCREEN_HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    minifbwin.limit_update_rate(Some(std::time::Duration::from_micros(20750)));

    while minifbwin.is_open() && !minifbwin.is_key_down(Key::Escape) {
        elapsed_time += 0.04f64;
        let mut buffer: Vec<u32> = vec![0x00000000; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];

        let mut rot_z = Mat4x4 { m: [[0f64; 4]; 4] };
        let mut rot_x = Mat4x4 { m: [[0f64; 4]; 4] };
        let f_theta = 1f64 * elapsed_time;

        rot_z.m[0][0] = f_theta.cos();
        rot_z.m[0][1] = f_theta.sin();
        rot_z.m[1][0] = -f_theta.sin();
        rot_z.m[1][1] = f_theta.cos();
        rot_z.m[2][2] = 1f64;
        rot_z.m[3][3] = 1f64;

        rot_x.m[0][0] = 1f64;
        rot_x.m[1][1] = (f_theta * 0.5f64).cos();
        rot_x.m[1][2] = (f_theta * 0.5f64).sin();
        rot_x.m[2][1] = -(f_theta * 0.5f64).sin();
        rot_x.m[2][2] = (f_theta * 0.5f64).cos();
        rot_x.m[3][3] = 1f64;

        for tr in mesh_cube.tris.iter() {
            let mut tri_projected = build_triangle([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
            let mut tri_translated;
            let mut tri_rotated_z = tr.clone();
            let mut tri_rotated_zx = tr.clone();

            tri_rotated_z.p[0] = multiply_matvec(&tr.p[0], &rot_z);
            tri_rotated_z.p[1] = multiply_matvec(&tr.p[1], &rot_z);
            tri_rotated_z.p[2] = multiply_matvec(&tr.p[2], &rot_z);

            tri_rotated_zx.p[0] = multiply_matvec(&tri_rotated_z.p[0], &rot_x);
            tri_rotated_zx.p[1] = multiply_matvec(&tri_rotated_z.p[1], &rot_x);
            tri_rotated_zx.p[2] = multiply_matvec(&tri_rotated_z.p[2], &rot_x);

            // offset into the screen
            tri_translated = tri_rotated_zx.clone();
            tri_translated.p[0].z = tr.p[0].z + 3f64;
            tri_translated.p[1].z = tr.p[1].z + 3f64;
            tri_translated.p[2].z = tr.p[2].z + 3f64;

            let mut normal = build_vertex(0f64, 0f64, 0f64);
            let mut line1 = build_vertex(0f64, 0f64, 0f64);
            let mut line2 = build_vertex(0f64, 0f64, 0f64);

            line1.x = tri_translated.p[1].x - tri_translated.p[0].x;
            line1.y = tri_translated.p[1].y - tri_translated.p[0].y;
            line1.z = tri_translated.p[1].z - tri_translated.p[0].z;

            line2.x = tri_translated.p[2].x - tri_translated.p[0].x;
            line2.y = tri_translated.p[2].y - tri_translated.p[0].y;
            line2.z = tri_translated.p[2].z - tri_translated.p[0].z;

            normal.x = line1.y * line2.z - line1.z * line2.y;
            normal.y = line1.z * line2.x - line1.x * line2.z;
            normal.z = line1.x * line2.y - line1.y * line2.x;

            // It's normally normal to normalise the normal - javidx9
            let l = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
            normal.x /= l;
            normal.y /= l;
            normal.z /= l;

            if normal.x * (tri_translated.p[0].x - camera.x)
                + normal.y * (tri_translated.p[0].y - camera.y)
                + normal.z * (tri_translated.p[0].z - camera.z) < 0f64 {
                // project from 3D -> 2D
                tri_projected.p[0] = multiply_matvec(&tri_translated.p[0], &mat_proj);
                tri_projected.p[1] = multiply_matvec(&tri_translated.p[1], &mat_proj);
                tri_projected.p[2] = multiply_matvec(&tri_translated.p[2], &mat_proj);

                tri_projected.p[0].x += 1f64;
                tri_projected.p[0].y += 1f64;
                tri_projected.p[1].x += 1f64;
                tri_projected.p[1].y += 1f64;
                tri_projected.p[2].x += 1f64;
                tri_projected.p[2].y += 1f64;

                tri_projected.p[0].x *= 0.5 * SCREEN_WIDTH;
                tri_projected.p[0].y *= 0.5 * SCREEN_HEIGHT;
                tri_projected.p[1].x *= 0.5 * SCREEN_WIDTH;
                tri_projected.p[1].y *= 0.5 * SCREEN_HEIGHT;
                tri_projected.p[2].x *= 0.5 * SCREEN_WIDTH;
                tri_projected.p[2].y *= 0.5 * SCREEN_HEIGHT;

                draw_triangle(&mut buffer,
                    [ 
                        Point {
                            x: tri_projected.p[0].x as i32,
                            y: tri_projected.p[0].y as i32
                        },
                        Point {
                            x: tri_projected.p[1].x as i32,
                            y: tri_projected.p[1].y as i32
                        },
                        Point {
                            x: tri_projected.p[2].x as i32,
                            y: tri_projected.p[2].y as i32
                        } 
                    ], 0x00fbfbfb
                );
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        minifbwin
            .update_with_buffer(&buffer, SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize)
            .unwrap();
    }
}
