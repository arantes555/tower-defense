use super::utils;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct Segment {
    pub a: Point,
    pub b: Point,
}

pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    fn from_points(p1: &Point, p2: &Point) -> Vector {
        return Vector {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
        };
    }

    fn cross_product(v1: &Vector, v2: &Vector) -> f64 {
        return (v1.x * v2.y) - (v1.y * v2.x);
    }

    fn dot_product(v1: &Vector, v2: &Vector) -> f64 {
        return (v1.x * v2.x) + (v1.y * v2.y);
    }
}

pub fn distance_points_squared(p1: &Point, p2: &Point) -> f64 {
    return ((p1.x - p2.x) * (p1.x - p2.x)) + ((p1.y - p2.y) * (p1.y - p2.y));
}

pub fn distance_point_segment_squared(p: &Point, s: &Segment) -> f64 {
    let dx = s.b.x - s.a.x;
    let dy = s.b.y - s.a.y;

    let dpx = p.x - s.a.x;
    let dpy = p.y - s.a.y;

    let seg_len_squared = (dx * dx) + (dy * dy); // we already have the deltas, because we need them afterwards anyway, so no need to use the function distance_points_squared

    if utils::are_values_equal(seg_len_squared, 0.0) {
        return dpx * dpx + dpy * dpy;
    } else {
        let t = ((dpx * dx) + (dpy * dy)) / seg_len_squared;
        if t <= 0.0 {
            // intersects at or to the "left" of first segment vertex (s.a).
            return distance_points_squared(&p, &s.a);
        } else if t >= 1.0 {
            // intersects at or to the "right" of second segment vertex (s.b).
            return distance_points_squared(&p, &s.b);
        } else {
            // The projection of the point to the point on the segment that is perpendicular succeeded and the point
            // is 'within' the bounds of the segment.
            // Set the intersection point as that projected point.
            let q = Point {
                x: ((1.0 - t) * s.a.x) + (t * s.b.x),
                y: ((1.0 - t) * s.a.y) + (t * s.b.y),
            };
            return distance_points_squared(&p, &q);
        }
    }
}

fn on_segment(s: &Segment, p: &Point) -> bool {
    return f64::min(s.a.x, s.b.x) <= p.x &&
        p.x <= f64::max(s.a.x, s.b.x) &&
        f64::min(s.a.y, s.b.y) <= p.y &&
        p.y <= f64::max(s.a.y, s.b.y);
}

pub fn segments_intersect(s1: &Segment, s2: &Segment) -> bool {
    // https://algorithmtutor.com/Computational-Geometry/Check-if-two-line-segment-intersect/
    let d1 = Vector::cross_product(&Vector::from_points(&s2.a, &s1.a), &Vector::from_points(&s2.a, &s2.b));
    let d2 = Vector::cross_product(&Vector::from_points(&s2.a, &s1.b), &Vector::from_points(&s2.a, &s2.b));
    let d3 = Vector::cross_product(&Vector::from_points(&s1.a, &s2.a), &Vector::from_points(&s1.a, &s1.b));
    let d4 = Vector::cross_product(&Vector::from_points(&s1.a, &s2.b), &Vector::from_points(&s1.a, &s1.b));

    if ((d1 < 0.0 && d2 > 0.0) || (d1 > 0.0 && d2 < 0.0)) && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0)) { // checking if segments cross each other
        return true;
    } else if utils::are_values_equal(d1, 0.0) && on_segment(s2, &s1.a) { // checking if a segment has one of the endpoints on the other
        return true;
    } else if utils::are_values_equal(d2, 0.0) && on_segment(s2, &s1.b) {
        return true;
    } else if utils::are_values_equal(d3, 0.0) && on_segment(s1, &s2.a) {
        return true;
    } else if utils::are_values_equal(d4, 0.0) && on_segment(s1, &s2.b) {
        return true;
    }

    return false;
}

pub fn distance_segments_squared(s1: &Segment, s2: &Segment) -> f64 {
    // https://stackoverflow.com/a/11427699
    // check to make sure both segments are long enough (i.e. verts are farther apart than minimum allowed vert distance).
    // If 1 or both segments are shorter than this min length, treat them as a single point.
    let s1len = distance_points_squared(&s1.a, &s1.b);
    if s1len < utils::EPSILON {
        return distance_point_segment_squared(&s1.a, s2);
    }
    let s2len = distance_points_squared(&s2.a, &s2.b);
    if s2len < utils::EPSILON {
        return distance_point_segment_squared(&s2.a, s1);
    }

    // cross product of the vectors of each segment
    let cross = Vector::cross_product(&Vector::from_points(&s1.a, &s1.b), &Vector::from_points(&s2.a, &s2.b));
    let parallel = utils::are_values_equal(cross, 0.0);

    if !parallel {
        let intersect = segments_intersect(s1, s2);
        if intersect {
            return 0.0;
        }
    }

    let d_s1a_s2 = distance_point_segment_squared(&s1.a, s2);
    let d_s1b_s2 = distance_point_segment_squared(&s1.b, s2);
    let mut min = f64::min(d_s1a_s2, d_s1b_s2);
    let d_s2a_s1 = distance_point_segment_squared(&s2.a, s1);
    min = f64::min(min, d_s2a_s1);
    let d_s2b_s1 = distance_point_segment_squared(&s2.b, s1);
    min = f64::min(min, d_s2b_s1);

    return min;
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::utils;

    #[test]
    fn test_distance_points_squared() {
        let d = distance_points_squared(
            &Point { x: 0.0, y: 1.0 },
            &Point { x: 1.0, y: 0.0 },
        );
        assert!(utils::are_values_equal(d, 2.0)); // supposed to be 2 (sqrt(2) squared)
    }
    
    #[test]
    fn test_distance_point_segment_squared() {
        let s = Segment {
            a: Point { x: 0.0, y: 1.0 },
            b: Point { x: 1.0, y: 0.0 },
        };

        let d = distance_point_segment_squared(&Point { x: 0.0, y: 0.0 }, &s);
        assert!(utils::are_values_equal(d, 0.5)); // supposed to be 0.5 (sqrt(2)/2 squared)

        let d = distance_point_segment_squared(&Point { x: 0.0, y: 1.0 }, &s);
        assert!(utils::are_values_equal(d, 0.0)); // supposed to be 0

        let d = distance_point_segment_squared(&Point { x: -1.0, y: 2.0 }, &s);
        assert!(utils::are_values_equal(d, 2.0)); // supposed to be 2

        let d = distance_point_segment_squared(&Point { x: 1.0, y: 1.0 }, &s);
        assert!(utils::are_values_equal(d, 0.5)); // supposed to be 0.5

        let d = distance_point_segment_squared(&Point { x: 2.0, y: 0.0 }, &s);
        assert!(utils::are_values_equal(d, 1.0)); // supposed to be 1
    }
    
    #[test]
    fn test_segments_intersect() {
        let intersect = segments_intersect(
            &Segment {
                a: Point { x: 0.0, y: 0.0 },
                b: Point { x: 1.0, y: 1.0 },
            },
            &Segment {
                a: Point { x: 1.0, y: 0.0 },
                b: Point { x: 0.0, y: 1.0 },
            },
        );
        assert_eq!(intersect, true);

        let intersect = segments_intersect(
            &Segment {
                a: Point { x: 0.0, y: 0.0 },
                b: Point { x: 0.0, y: 1.0 },
            },
            &Segment {
                a: Point { x: 0.0, y: 0.0 },
                b: Point { x: 1.0, y: 0.0 },
            },
        );
        assert_eq!(intersect, true);

        let intersect = segments_intersect(
            &Segment {
                a: Point { x: 0.0, y: 0.0 },
                b: Point { x: 0.0, y: 1.0 },
            },
            &Segment {
                a: Point { x: 0.1, y: 0.0 },
                b: Point { x: 1.0, y: 0.0 },
            },
        );
        assert_eq!(intersect, false);
    }

    #[test]
    fn test_distance_segments_squared() {
        let d = distance_segments_squared(
            &Segment {
                a: Point { x: 0.0, y: 0.0 },
                b: Point { x: 1.0, y: 1.0 },
            },
            &Segment {
                a: Point { x: 1.0, y: 0.0 },
                b: Point { x: 0.0, y: 1.0 },
            },
        );
        assert!(utils::are_values_equal(d, 0.0)); // supposed to be 0


        let d = distance_segments_squared(
            &Segment {
                a: Point { x: 0.0, y: 0.0 },
                b: Point { x: 0.0, y: 1.0 },
            },
            &Segment {
                a: Point { x: 0.1, y: 0.0 },
                b: Point { x: 1.0, y: 0.0 },
            },
        );
        assert!(utils::are_values_equal(d, 0.01)); // supposed to be 0.01 (0.1 squared)

        let d = distance_segments_squared(
            &Segment {
                a: Point { x: 0.0, y: 0.0 },
                b: Point { x: 0.0, y: 1.0 },
            },
            &Segment {
                a: Point { x: 1.0, y: 0.0 },
                b: Point { x: 1.0, y: 1.0 },
            },
        );
        assert!(utils::are_values_equal(d, 1.0)); // supposed to be 1
    }
}
