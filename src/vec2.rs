use auto_ops::{impl_op_ex, impl_op_ex_commutative};

#[derive(Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl Vec2<f64> {
    pub fn new() -> Vec2<f64> {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn rotate(&mut self, radians: f64) {
        *self = Vec2 {
            x: self.x * radians.cos() - self.y * radians.sin(),
            y: self.x * radians.sin() + self.y * radians.cos(),
        };
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn as_usize(&self) -> Vec2<usize> {
        Vec2 {
            x: self.x as usize,
            y: self.y as usize,
        }
    }
}

impl_op_ex!(+ |lhs: &Vec2<f64>, rhs: &Vec2<f64>| -> Vec2<f64> { Vec2 { x: lhs.x + rhs.x, y: lhs.y + rhs.y } });
impl_op_ex!(-|lhs: &Vec2<f64>, rhs: &Vec2<f64>| -> Vec2<f64> {
    Vec2 {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
    }
});
impl_op_ex!(+= |lhs: &mut Vec2<f64>, rhs: &Vec2<f64>| { *lhs = Vec2 { x: lhs.x + rhs.x, y: lhs.y + rhs.y } });
impl_op_ex!(-= |lhs: &mut Vec2<f64>, rhs: &Vec2<f64>| { *lhs = Vec2 { x: lhs.x - rhs.x, y: lhs.y - rhs.y } });
impl_op_ex_commutative!(*|lhs: &Vec2<f64>, rhs: &f64| -> Vec2<f64> {
    Vec2 {
        x: lhs.x * rhs,
        y: lhs.y * rhs,
    }
});
impl_op_ex!(/ |lhs: &Vec2<f64>, rhs: &f64| -> Vec2<f64> { lhs * (1.0 / rhs) });
