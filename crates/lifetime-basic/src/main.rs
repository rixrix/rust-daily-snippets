struct Path<'a> {
    point_x: &'a i32,
    point_y: &'a i32,
}

fn main() {
    let px = 3200;
    let py = (px / 2) as i32;
    let maze = Path {
        point_x: &px,
        point_y: &py,
    };

    println!("x = {}, y = {}", maze.point_x, maze.point_y);
}
