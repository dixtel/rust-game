use crate::utils::*;

pub fn in_square(p: &Position, start: &Position, size: usize) -> bool {
    if p.x >= start.x
        && p.x <= start.x + size as f64
        && p.y >= start.y
        && p.y <= start.y + size as f64
    {
        true
    } else {
        false
    }
}
