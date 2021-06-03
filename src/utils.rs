pub const EPSILON: f64 = 1e-8;

pub fn are_values_equal(x: f64, y: f64) -> bool {
    if x >= (y - EPSILON) && x <= (y + EPSILON) {
        return true;
    }
    return false;
}
