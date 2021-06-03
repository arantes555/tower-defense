mod geometry;
mod utils;

fn main() {
    let p1 = geometry::Point {
        x: 0.0,
        y: 1.0,
    };
    let p2 = geometry::Point {
        x: 1.0,
        y: 0.0,
    };
    let d_pp = geometry::distance_points_squared(&p1, &p2);
    println!("Distance (squared) between p1 and p2 is {}", d_pp); // supposed to be 2 (sqrt(2) squared)

    let s = geometry::Segment {
        a: p1,
        b: p2,
    };

    let d_ps = geometry::distance_point_segment_squared(&geometry::Point { x: 0.0, y: 0.0 }, &s);
    println!("Distance (squared) between s and 0;0 is {}", d_ps); // supposed to be 0.5 (sqrt(2)/2 squared)

    let d_ps = geometry::distance_point_segment_squared(&geometry::Point { x: 0.0, y: 1.0 }, &s);
    println!("Distance (squared) between s and 0;1 is {}", d_ps); // supposed to be 0

    let d_ps = geometry::distance_point_segment_squared(&geometry::Point { x: -1.0, y: 2.0 }, &s);
    println!("Distance (squared) between s and -1;2 is {}", d_ps); // supposed to be 2

    let d_ps = geometry::distance_point_segment_squared(&geometry::Point { x: 1.0, y: 1.0 }, &s);
    println!("Distance (squared) between s and 1;1 is {}", d_ps); // supposed to be 0.4

    let d_ps = geometry::distance_point_segment_squared(&geometry::Point { x: 2.0, y: 0.0 }, &s);
    println!("Distance (squared) between s and 2;0 is {}", d_ps); // supposed to be 1

    let intersect = geometry::segments_intersect(
        &geometry::Segment {
            a: geometry::Point { x: 0.0, y: 0.0 },
            b: geometry::Point { x: 1.0, y: 1.0 },
        },
        &geometry::Segment {
            a: geometry::Point { x: 1.0, y: 0.0 },
            b: geometry::Point { x: 0.0, y: 1.0 },
        },
    );
    println!("Segments 0;0->1;1 & 1;0->0;1 intersect : {}", intersect); // supposed to be true

    let intersect = geometry::segments_intersect(
        &geometry::Segment {
            a: geometry::Point { x: 0.0, y: 0.0 },
            b: geometry::Point { x: 0.0, y: 1.0 },
        },
        &geometry::Segment {
            a: geometry::Point { x: 0.0, y: 0.0 },
            b: geometry::Point { x: 1.0, y: 0.0 },
        },
    );
    println!("Segments 0;0->0;1 & 0;0->1;0 intersect : {}", intersect); // supposed to be true

    let intersect = geometry::segments_intersect(
        &geometry::Segment {
            a: geometry::Point { x: 0.0, y: 0.0 },
            b: geometry::Point { x: 0.0, y: 1.0 },
        },
        &geometry::Segment {
            a: geometry::Point { x: 0.1, y: 0.0 },
            b: geometry::Point { x: 1.0, y: 0.0 },
        },
    );
    println!("Segments 0.1;0->0;1 & 0;0->1;0 intersect : {}", intersect); // supposed to be false

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

    let d = geometry::distance_segments_squared(
        &geometry::Segment {
            a: geometry::Point { x: 0.0, y: 0.0 },
            b: geometry::Point { x: 0.0, y: 1.0 },
        },
        &geometry::Segment {
            a: geometry::Point { x: 0.1, y: 0.0 },
            b: geometry::Point { x: 1.0, y: 0.0 },
        },
    );
    println!("Distance between Segments 0;0->0;1 & 0;0->1;0 : {}", d); // supposed to be 0.01 (0.1 squared)

    let d = geometry::distance_segments_squared(
        &geometry::Segment {
            a: geometry::Point { x: 0.0, y: 0.0 },
            b: geometry::Point { x: 0.0, y: 1.0 },
        },
        &geometry::Segment {
            a: geometry::Point { x: 1.0, y: 0.0 },
            b: geometry::Point { x: 1.0, y: 1.0 },
        },
    );
    println!("Distance between Segments 0;0->0;1 & 1;0->1;1 : {}", d); // supposed to be 1
}
