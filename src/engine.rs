extern crate bresenham;

//const SCREEN_HEIGHT: i32 = 720;
const SCREEN_WIDTH: i32 = 1280;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

pub fn get_point(p: Point) -> i32 {
    (p.y * SCREEN_WIDTH) + (p.x + 1)
}

pub fn draw_triangle(buffer: &mut Vec<u32>, points: [Point; 3], color: u32){
    draw_line(buffer, [points[0].x, points[0].y, points[1].x, points[1].y], color);
    draw_line(buffer, [points[1].x, points[1].y, points[2].x, points[2].y], color);
    draw_line(buffer, [points[0].x, points[0].y, points[2].x, points[2].y], color);
}

pub fn draw_line(buffer: &mut Vec<u32>, coords: [i32; 4], color: u32){
    let start = Point {x: coords[0], y: coords[1]};
    let target = Point {x: coords[2], y: coords[3]};

    for (x, y) in bresenham::Bresenham::new((start.x as isize, start.y as isize), (target.x as isize, target.y as isize)) {
        buffer[get_point(Point {x: x as i32, y: y as i32}) as usize] = color;
    }
}