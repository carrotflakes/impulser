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
            if !src.point.left_side_of_line([ps[0], ps[1]]) {
                return None;
            }

            let [a, b] = [ps[0], ps[1]];

            let aa = (a - src.point).atan2();
            let ba = (b - src.point).atan2();

            let [aa, ba] = if (aa - ba).abs() < PI {
                [aa.min(ba), aa.max(ba)]
            } else {
                [aa.max(ba), aa.min(ba) + TAU]
            };

            // intersect angle
            let intersect_angle_range = if let Some(angle_range) = &src.angle_range {
                if aa < angle_range.start {
                    if angle_range.start < ba {
                        angle_range.start..ba.min(angle_range.end)
                    } else {
                        return None;
                    }
                } else if aa < angle_range.end {
                    aa..ba.min(angle_range.end)
                } else {
                    return None;
                }
            } else {
                aa..ba
            };

            // symmetric angle
            let a = (b - a).atan2() * 2.0;
            let mut angle_range = a - intersect_angle_range.end..a - intersect_angle_range.start;
            if angle_range.start < -PI {
                angle_range = angle_range.start + TAU..angle_range.end + TAU;
            } else if PI < angle_range.end {
                angle_range = angle_range.start - TAU..angle_range.end - TAU;
            }

            Some(TruncatedWave {
                point: src.point.symmetric_point([ps[0], ps[1]]),
                angle_range: Some(angle_range),
            })
        })
    }
}
