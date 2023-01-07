use std::f64::consts::{PI, TAU};

use crate::model::*;

#[derive(Debug, Clone)]
pub struct Room {
    pub(crate) closed_points: Vec<Point>,
}

impl Room {
    pub fn new(mut points: Vec<Point>) -> Self {
        points.push(points[0]);
        Self {
            closed_points: points,
        }
    }

    pub fn hit<'a>(&'a self, src: TruncatedWave) -> impl Iterator<Item = TruncatedWave> + 'a {
        self.closed_points.windows(2).filter_map(move |ps| {
            let [p1, p2] = [ps[0], ps[1]];

            if !src.point.is_left_side_of_line([p1, p2]) {
                return None;
            }

            let a1 = (p1 - src.point).atan2();
            let a2 = (p2 - src.point).atan2();

            let [a1, a2] = if (a1 - a2).abs() < PI {
                [a1.min(a2), a1.max(a2)]
            } else {
                [a1.max(a2), a1.min(a2) + TAU]
            };

            // intersect angle
            let intersect_angle_range = if let Some(angle_range) = &src.angle_range {
                if a1 < angle_range.start {
                    if angle_range.start < a2 {
                        angle_range.start..a2.min(angle_range.end)
                    } else {
                        return None;
                    }
                } else if a1 < angle_range.end {
                    a1..a2.min(angle_range.end)
                } else {
                    return None;
                }
            } else {
                a1..a2
            };

            // symmetric angle
            let a = (p2 - p1).atan2() * 2.0;
            let mut angle_range = a - intersect_angle_range.end..a - intersect_angle_range.start;
            if angle_range.start < -PI {
                angle_range = angle_range.start + TAU..angle_range.end + TAU;
            } else if PI < angle_range.end {
                angle_range = angle_range.start - TAU..angle_range.end - TAU;
            }

            Some(TruncatedWave {
                point: src.point.symmetric_point([p1, p2]),
                angle_range: Some(angle_range),
            })
        })
    }
}
