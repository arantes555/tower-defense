mod geometry;
mod utils;

fn main() {
    let d = geometry::distance_points_squared(
        &geometry::Point { x: 0.0, y: 1.0 },
        &geometry::Point { x: 1.0, y: 0.0 },
    );
    println!("Distance (squared) between p1 and p2 is {}", d);

    let s = geometry::Segment {
        a: geometry::Point { x: 0.0, y: 1.0 },
        b: geometry::Point { x: 1.0, y: 0.0 },
    };

    let d = geometry::distance_point_segment_squared(&geometry::Point { x: 0.0, y: 0.0 }, &s);
    println!("Distance (squared) between s and 0;0 is {}", d); // supposed to be 0.5 (sqrt(2)/2 squared)

    let d = geometry::distance_segments_squared(
        &geometry::Segment {
            a: geometry::Point { x: 0.0, y: 0.0 },
            b: geometry::Point { x: 1.0, y: 1.0 },
        },
        &geometry::Segment {
            a: geometry::Point { x: 1.0, y: 0.0 },
            b: geometry::Point { x: 0.0, y: 1.0 },
        },
    );
    println!("Distance between Segments 0;0->1;1 & 1;0->0;1 : {}", d); // supposed to be 0
}
