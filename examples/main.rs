use impulser::{
    model::{Point, TruncatedWave},
    render::Render,
};

fn main() {
    let mut points = vec![
        Point::new(1.0, -1.0),
        Point::new(1.0, 1.0),
        Point::new(-1.0, 1.0),
        Point::new(-1.0, -0.1),
    ];
    points.push(points[0]);
    let room = impulser::room::Room::new(points);
    let src = Point::new(0.0, 0.0);
    let dst = Point::new(0.0, 0.5);
    Render {
        dst,
        cutoff_distance: 3.0,
        hit: |src: TruncatedWave| room.hit(src),
    }
    .render(src.into(), &mut |src| {
        println!("{:?}", src.distance_with(dst));
    });
}
