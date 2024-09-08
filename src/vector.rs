pub fn dot(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    x1 * x2 + y1 * y2
}

pub fn proj(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
    let r = dot(x1, y1, x2, y2);
    let r2 = dot(x2, y2, x2, y2);
    (x2 * r / r2, y2 * r / r2)
}

pub fn dist2(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let x = x1 - x2;
    let y = y1 - y2;

    dot(x, y, x, y)
}

pub fn norm(x: f64, y: f64, len: f64) -> (f64, f64) {
    let r = (x * x + y * y).sqrt();
    (x / r * len, y / r * len)
}