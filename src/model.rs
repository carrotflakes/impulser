use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub struct Point([f64; 2]);

impl Point {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Point([x, y])
    }

    #[inline]
    pub fn atan2(&self) -> f64 {
        self.0[1].atan2(self.0[0])
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn is_left_side_of_line(&self, line: [Point; 2]) -> bool {
        let [a, b] = line;
        let nb = b - a;
        let ns = *self - a;
        let y = ns.0[0] * nb.0[1] - ns.0[1] * nb.0[0];
        y < 0.0
    }

    pub fn symmetric_point(&self, line: [Point; 2]) -> Self {
        let [a, b] = line;
        let nb = b - a;
        let ns = *self - a;
        let x = ns.0[0] * nb.0[0] + ns.0[1] * nb.0[1];
        let y = ns.0[0] * -nb.0[1] + ns.0[1] * nb.0[0];
        let y = -y;

        let scale = nb.0[0].powi(2) + nb.0[1].powi(2);
        Point([
            (x * nb.0[0] - y * nb.0[1]) / scale + a.0[0],
            (x * nb.0[1] + y * nb.0[0]) / scale + a.0[1],
        ])
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1]])
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
        (self.point.0[0] - dst.0[0]).hypot(self.point.0[1] - dst.0[1])
    }

    pub fn hit_with_point(&self, dst: Point) -> bool {
        let vec = dst - self.point;
        let angle = vec.0[1].atan2(vec.0[0]);
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
