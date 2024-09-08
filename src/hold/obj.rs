use std::collections::HashMap;

pub trait Obj {
    fn hold_location(&mut self, x: f64, y: f64);
    fn throw(&mut self, vx: f64, vy: f64);
    fn grab(&self, x: f64, y: f64) -> f64;
    fn hold(&mut self) -> (f64, f64);
}

impl dyn Obj {
    pub fn nearest(objs: &mut HashMap<u64, &mut dyn Obj>, x: f64, y: f64, min: f64, not: u64) -> Option<u64> {
        let mut mindst = min;
        let mut f = None;

        for (id, obj) in objs {
            let dst = obj.grab(x, y);
            if mindst > dst && not != id.clone() {
                mindst = dst;
                f = Some(id.clone())
            }
        }

        f
    }
}