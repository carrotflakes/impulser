use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub struct Point([f64; 2]);

impl Point {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Point([x, y])
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    #[inline]
    pub fn atan2(&self) -> f64 {
        self.y().atan2(self.x())
    }

    pub fn is_left_side_of_line(&self, line: [Point; 2]) -> bool {
        let [a, b] = line;
        let nb = b - a;
        let ns = *self - a;
        (ns.x() * nb.y() - ns.y() * nb.x()) < 0.0
    }

    pub fn symmetric_point(&self, line: [Point; 2]) -> Self {
        let [a, b] = line;
        let nb = b - a;
        let ns = *self - a;
        let x = ns.x() * nb.x() + ns.y() * nb.y();
        let y = ns.x() * -nb.y() + ns.y() * nb.x();
        let y = -y;

        let scale = nb.x().powi(2) + nb.y().powi(2);
        Point([
            (x * nb.x() - y * nb.y()) / scale + a.x(),
            (x * nb.y() + y * nb.x()) / scale + a.y(),
        ])
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point([self.x() - rhs.x(), self.y() - rhs.y()])
    }
}

#[derive(Debug, Clone)]
pub struct TruncatedWave {
    pub point: Point,
    pub angle_range: Option<Range<f64>>,
}

impl TruncatedWave {
    #[inline]
    pub fn distance_with(&self, dst: Point) -> f64 {
        (self.point.x() - dst.x()).hypot(self.point.y() - dst.y())
    }

    pub fn hit_with_point(&self, dst: Point) -> bool {
        let vec = dst - self.point;
        let angle = vec.y().atan2(vec.x());
        if let Some(range) = &self.angle_range {
            range.contains(&angle)
        } else {
            true
        }
    }
}

impl From<Point> for TruncatedWave {
    fn from(point: Point) -> Self {
        TruncatedWave {
            point,
            angle_range: None,
        }
    }
}

#[test]
fn test() {
    dbg!(Point::new(2.0, 3.0).symmetric_point([Point::new(0.0, 0.0), Point::new(1.0, 1.0)]));
    assert!(Point::new(2.0, 3.0).is_left_side_of_line([Point::new(0.0, 0.0), Point::new(1.0, 1.0)]));
}
