use image::{RgbImage, Rgb};
use day_9::utils::get_input;

fn point_in_polygon(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (px, py) = point;
    let n = polygon.len();

    let point_on_segment = |p: (i64, i64), a: (i64, i64), b: (i64, i64)| -> bool {
        let (px, py) = p;
        let (x0, y0) = a;
        let (x1, y1) = b;
        let cross = (px - x0) * (y1 - y0) - (py - y0) * (x1 - x0);
        if cross != 0 {
            return false;
        }
        let min_x = x0.min(x1);
        let max_x = x0.max(x1);
        let min_y = y0.min(y1);
        let max_y = y0.max(y1);
        px >= min_x && px <= max_x && py >= min_y && py <= max_y
    };

    for i in 0..n {
        let (x0, y0) = polygon[i];
        let (x1, y1) = polygon[(i + 1) % n];
        if point_on_segment(point, (x0, y0), (x1, y1)) {
            return true;
        }
    }

    let mut inside = false;
    for i in 0..n {
        let (x0, y0) = polygon[i];
        let (x1, y1) = polygon[(i + 1) % n];
        if ((y0 > py) != (y1 > py)) &&
           (px < (x1 - x0) * (py - y0) / (y1 - y0) + x0) {
            inside = !inside;
        }
    }
    inside
}

fn main() {
    let polygon: Vec<(i64, i64)> = get_input("src/input.txt")
        .iter()
        .map(|&(x, y)| (x as i64, y as i64))
        .collect();

    let min_x = polygon.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = polygon.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = polygon.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = polygon.iter().map(|(_, y)| *y).max().unwrap();

    let img_width = 800;
    let img_height = 600;
    let scale_x = (img_width - 20) as f64 / (max_x - min_x) as f64;
    let scale_y = (img_height - 20) as f64 / (max_y - min_y) as f64;
    let scale = scale_x.min(scale_y);
    let offset_x = 10.0 - min_x as f64 * scale;
    let offset_y = 10.0 - min_y as f64 * scale;

    let map_point = |(x, y): (i64, i64)| -> (i64, i64) {
        let ix = (x as f64 * scale + offset_x).round() as i64;
        let iy = (y as f64 * scale + offset_y).round() as i64;
        (ix, iy)
    };

    let mut img = RgbImage::new(img_width, img_height);
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    for i in 0..polygon.len() {
        let (x1, y1) = map_point(polygon[i]);
        let (x2, y2) = map_point(polygon[(i + 1) % polygon.len()]);
        draw_line(&mut img, x1, y1, x2, y2, Rgb([0, 0, 0]));
    }

    let points_to_mark = vec![(94654, 50355)];
    let threshold_y = points_to_mark[0].1;

    for &(px, py) in &polygon {
        if py >= threshold_y {
            let (x, y) = map_point((px, py));
            if x >= 0 && x < img_width as i64 && y >= 0 && y < img_height as i64 {
                img.put_pixel(x as u32, y as u32, Rgb([255, 0, 0]));
            }
        }
    }

    let marked_point = (94654, 50355);
    let mut max_area = 0;
    let mut best_corner = marked_point;

    for &(x, y) in &polygon {
        if y >= marked_point.1 {
            let left = marked_point.0.min(x);
            let right = marked_point.0.max(x);
            let bottom = marked_point.1;
            let top = y;

            let corners = [
                (left, bottom),
                (right, bottom),
                (right, top),
                (left, top),
            ];

            if corners.iter().all(|&c| point_in_polygon(c, &polygon)) {
                let width = (right as i64 - left as i64 + 1) as u64;
                let height = (top as i64 - bottom as i64 + 1) as u64;
                let area = width * height;
                if area > max_area {
                    max_area = area;
                    best_corner = (x, y);
                }
            }
        }
    }

    println!("{}", max_area);

    let (x0, y0) = map_point(marked_point);
    let (x1, y1) = map_point(best_corner);

    draw_line(&mut img, x0, y0, x1, y0, Rgb([0, 0, 255]));
    draw_line(&mut img, x1, y0, x1, y1, Rgb([0, 0, 255]));
    draw_line(&mut img, x1, y1, x0, y1, Rgb([0, 0, 255]));
    draw_line(&mut img, x0, y1, x0, y0, Rgb([0, 0, 255]));

    img.save("polygon.png").unwrap();
}

fn draw_line(img: &mut RgbImage, x0: i64, y0: i64, x1: i64, y1: i64, color: Rgb<u8>) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && x0 < img.width() as i64 && y0 >= 0 && y0 < img.height() as i64 {
            img.put_pixel(x0 as u32, y0 as u32, color);
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}
