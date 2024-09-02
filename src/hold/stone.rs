use super::obj::Obj;

struct Stone {
    x: f64,
    y: f64,
}

impl Stone {
    
}

impl Obj for Stone {
    fn hold_location(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}